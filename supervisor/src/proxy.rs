use anyhow::{Context, Result};
use axum::async_trait;
use bytes::Bytes;
use hyper::header;
use pingora::http::ResponseHeader;
use pingora::protocols::http::error_resp;
use pingora::proxy::{http_proxy_service, ProxyHttp, Session, FailToProxy};
use pingora::server::configuration::{Opt, ServerConf};
use pingora::services::Service;
use pingora::upstreams::peer::HttpPeer;
use pingora::{Error, ErrorSource, ErrorType, OrErr};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;

/// A proxy that routes requests to various services and provides health checking capabilities
#[derive(Clone)]
pub struct GatewayProxy {
    /// Map of service names to their local ports
    services: HashMap<String, u16>,
    /// Upstream server address for proxying requests
    upstream: SocketAddr,
    /// HTTP client for making health check requests
    client: reqwest::Client,
}

/// Represents the health status of a service or the overall system
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    /// Service is healthy and operating normally
    Ok,
    /// Service is unhealthy or experiencing issues
    Unhealthy,
}

/// Response structure for the health check endpoint
#[derive(Clone, Serialize, Deserialize)]
pub struct HealthzResponse {
    /// Current health status of the service
    #[serde(rename = "code")]
    pub status: HealthStatus,
    /// Human-readable message describing the health status
    pub message: String,
    /// Detailed information about the service's health
    pub details: HealthzDetails,
}

/// Detailed health information about the service
#[derive(Clone, Serialize, Deserialize)]
pub struct HealthzDetails {
    /// Application revision identifier
    pub app_revision: String,
    /// Version of the Encore compiler
    pub encore_compiler: String,
    /// Deployment identifier
    pub deploy_id: String,
    /// Results of individual health checks
    pub checks: Vec<HealthzCheckResult>,
    /// List of enabled experimental features
    pub enabled_experiments: Vec<String>,
}

/// Result of an individual health check
#[derive(Clone, Serialize, Deserialize)]
pub struct HealthzCheckResult {
    /// Name of the health check
    pub name: String,
    /// Whether the check passed
    pub passed: bool,
    /// Error message if the check failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GatewayProxy {
    /// Creates a new GatewayProxy instance
    ///
    /// # Arguments
    ///
    /// * `client` - HTTP client for making requests
    /// * `upstream` - Address of the upstream server
    /// * `services` - Map of service names to their local ports
    pub fn new(
        client: reqwest::Client,
        upstream: SocketAddr,
        services: HashMap<String, u16>,
    ) -> Self {
        GatewayProxy {
            client,
            upstream,
            services,
        }
    }

    pub async fn serve(self, listen_addr: String, token: CancellationToken) {
        let conf = Arc::new(
            ServerConf::new_with_opt_override(&Opt {
                upgrade: false,
                daemon: false,
                nocapture: false,
                test: false,
                conf: None,
            })
            .unwrap(),
        );
        let mut proxy = http_proxy_service(&conf, self);

        proxy.add_tcp(listen_addr.as_str());

        let (tx, rx) = watch::channel(false);

        tokio::select! {
            _ = proxy.start_service(
                #[cfg(unix)]
                None,
                rx,
                100
            ) => {},
            _ = token.cancelled() => {
                log::info!("Shutting down pingora proxy");
                tx.send(true).expect("failed to shutdown pingora");
            }
        }
    }

    // concurrently calls /__encore/healthz for all services. Returns "unhealthy" if any of them
    // does not return "ok".
    pub async fn health_check(&self) -> Result<HealthzResponse> {
        let handles = self.services.clone().into_iter().map(|(svc, port)| {
            let client = self.client.clone();
            let url = format!("http://127.0.0.1:{}/__encore/healthz", port);
            tokio::spawn(async move {
                let err_resp = || HealthzResponse {
                    status: HealthStatus::Unhealthy,
                    message: "healthcheck failed".to_string(),
                    details: HealthzDetails {
                        app_revision: "".to_string(),
                        encore_compiler: "".to_string(),
                        deploy_id: "".to_string(),
                        checks: vec![HealthzCheckResult {
                            name: format!("service.{}.initialized", svc),
                            passed: false,
                            error: None,
                        }],
                        enabled_experiments: vec![],
                    },
                };

                match client
                    .get(url.as_str())
                    .timeout(std::time::Duration::from_secs(5))
                    .send()
                    .await
                    .context(format!("failed to get health check for service {}", svc))
                    .and_then(|r| {
                        if r.status().is_success() {
                            Ok(r)
                        } else {
                            Err(anyhow::anyhow!("Service {} returned status {}", svc, r.status()))
                        }
                    }) {
                    Ok(res) => {
                        match res
                            .json::<HealthzResponse>()
                            .await
                            .context("Failed to parse response body")
                        {
                            Ok(res) => res,
                            Err(_) => err_resp(),
                        }
                    }
                    Err(_) => err_resp(),
                }
            })
        });

        let results: Vec<HealthzResponse> = futures::future::join_all(handles)
            .await
            .into_iter()
            .map(|r| r.context("failed future"))
            .collect::<Result<Vec<_>>>()
            .context("Failed to complete health checks for all services")?;

        results
            .iter()
            .fold(None::<HealthzResponse>, |rtn, resp| match rtn {
                Some(mut res) => {
                    if resp.status != HealthStatus::Ok {
                        res.status = HealthStatus::Unhealthy;
                        res.details.checks.extend(resp.details.checks.clone())
                    }
                    Some(res)
                }
                None => Some(resp.clone()),
            })
            .ok_or(anyhow::anyhow!("No results"))
    }
}

#[async_trait]
impl ProxyHttp for GatewayProxy {
    type CTX = Option<String>;

    fn new_ctx(&self) -> Self::CTX {
        None
    }

    // see https://github.com/cloudflare/pingora/blob/main/docs/user_guide/internals.md for
    // details on when different filters are called.

    async fn request_filter(
        &self,
        session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<bool>
    where
        Self::CTX: Send + Sync,
    {
        if session.req_header().uri.path() == "/__encore/healthz" {
            let healthz_resp = self
                .health_check()
                .await
                .or_err(ErrorType::HTTPStatus(503), "failed to run health check")?;
            let healthz_bytes: Vec<u8> = serde_json::to_vec(&healthz_resp)
                .or_err(ErrorType::HTTPStatus(503), "could not encode response")?;

            let code = if healthz_resp.status == HealthStatus::Ok { 200 } else { 503 };
            let mut header = ResponseHeader::build(code, None)?;
            header.insert_header(header::CONTENT_LENGTH, healthz_bytes.len())?;
            header.insert_header(header::CONTENT_TYPE, "application/json")?;
            session
                .write_response_header(Box::new(header), false)
                .await?;
            session
                .write_response_body(Some(Bytes::from(healthz_bytes)), true)
                .await?;

            return Ok(true);
        }
        Ok(false)
    }

    async fn upstream_peer(
        &self,
        _session: &mut Session,
        _ctx: &mut Self::CTX,
    ) -> pingora::Result<Box<HttpPeer>> {
        let peer: HttpPeer = HttpPeer::new(self.upstream, false, "localhost".to_string());
        Ok(Box::new(peer))
    }

    async fn fail_to_proxy(&self, session: &mut Session, e: &Error , _ctx: &mut Self::CTX) -> FailToProxy 
    where
        Self::CTX: Send + Sync,
    {
        // modified version of `Session::respond_error`

        let code = match e.etype() {
            ErrorType::HTTPStatus(code) => *code,
            _ => {
                match e.esource() {
                    ErrorSource::Upstream => 502,
                    ErrorSource::Downstream => {
                        match e.etype() {
                            ErrorType::WriteError
                            | ErrorType::ReadError
                            | ErrorType::ConnectionClosed => {
                                /* conn already dead */
                                return FailToProxy;
                            }
                            _ => 400,
                        }
                    }
                    ErrorSource::Internal | ErrorSource::Unset => 500,
                }
            }
        };

        let (resp, body) = (
            match code {
                /* common error responses are pre-generated */
                502 => error_resp::HTTP_502_RESPONSE.clone(),
                400 => error_resp::HTTP_400_RESPONSE.clone(),
                _ => error_resp::gen_error_response(code),
            },
            None,
        );
        session.set_keepalive(None);
        session
            .write_response_header(Box::new(resp), false)
            .await
            .unwrap_or_else(|e| {
                log::error!("failed to send error response to downstream: {e}");
            });

        session
            .write_response_body(body, true)
            .await
            .unwrap_or_else(|e| log::error!("failed to write body: {e}"));

        code
    }
}
