mod client;

mod server;


pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    // does nothing

    Ok(())
}

#[tonic::async_trait]
impl Daemon for MyDaemon {
    type CheckStream = tokio_stream::wrappers::ReceiverStream<Result<CommandMessage, Status>>;
    type RunStream = tokio_stream::wrappers::ReceiverStream<Result<CommandMessage, Status>>;
    type TestStream = tokio_stream::wrappers::ReceiverStream<Result<CommandMessage, Status>>;
    type ExecScriptStream = tokio_stream::wrappers::ReceiverStream<Result<CommandMessage, Status>>;
    type ExportStream = tokio_stream::wrappers::ReceiverStream<Result<CommandMessage, Status>>;
    type DBProxyStream = tokio_stream::wrappers::ReceiverStream<Result<CommandMessage, Status>>;
    type DBResetStream = tokio_stream::wrappers::ReceiverStream<Result<CommandMessage, Status>>;

    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<Self::CheckStream>, Status> {
        // ... your existing implementation ...
    }

    async fn run(
        &self,
        _request: Request<encore::daemon::RunRequest>,
    ) -> Result<Response<Self::RunStream>, Status> {
        unimplemented!()
    }

    async fn test(
        &self,
        _request: Request<encore::daemon::TestRequest>,
    ) -> Result<Response<Self::TestStream>, Status> {
        unimplemented!()
    }

    async fn test_spec(
        &self,
        _request: Request<encore::daemon::TestSpecRequest>,
    ) -> Result<Response<encore::daemon::TestSpecResponse>, Status> {
        unimplemented!()
    }

    async fn exec_script(
        &self,
        _request: Request<encore::daemon::ExecScriptRequest>,
    ) -> Result<Response<Self::ExecScriptStream>, Status> {
        unimplemented!()
    }

    async fn export(
        &self,
        _request: Request<encore::daemon::ExportRequest>,
    ) -> Result<Response<Self::ExportStream>, Status> {
        unimplemented!()
    }

    async fn db_connect(
        &self,
        _request: Request<encore::daemon::DbConnectRequest>,
    ) -> Result<Response<encore::daemon::DbConnectResponse>, Status> {
        unimplemented!()
    }

    async fn db_proxy(
        &self,
        _request: Request<encore::daemon::DbProxyRequest>,
    ) -> Result<Response<Self::DBProxyStream>, Status> {
        unimplemented!()
    }

    async fn db_reset(
        &self,
        _request: Request<encore::daemon::DbResetRequest>,
    ) -> Result<Response<Self::DBResetStream>, Status> {
        unimplemented!()
    }

    async fn gen_client(
        &self,
        _request: Request<encore::daemon::GenClientRequest>,
    ) -> Result<Response<encore::daemon::GenClientResponse>, Status> {
        unimplemented!()
    }

    async fn gen_wrappers(
        &self,
        _request: Request<encore::daemon::GenWrappersRequest>,
    ) -> Result<Response<encore::daemon::GenWrappersResponse>, Status> {
        unimplemented!()
    }

    async fn secrets_refresh(
        &self,
        _request: Request<encore::daemon::SecretsRefreshRequest>,
    ) -> Result<Response<encore::daemon::SecretsRefreshResponse>, Status> {
        unimplemented!()
    }

    async fn version(
        &self,
        _request: Request<()>,
    ) -> Result<Response<encore::daemon::VersionResponse>, Status> {
        unimplemented!()
    }

    async fn create_namespace(
        &self,
        _request: Request<encore::daemon::CreateNamespaceRequest>,
    ) -> Result<Response<encore::daemon::Namespace>, Status> {
        unimplemented!()
    }

    async fn switch_namespace(
        &self,
        _request: Request<encore::daemon::SwitchNamespaceRequest>,
    ) -> Result<Response<encore::daemon::Namespace>, Status> {
        unimplemented!()
    }

    async fn list_namespaces(
        &self,
        _request: Request<encore::daemon::ListNamespacesRequest>,
    ) -> Result<Response<encore::daemon::ListNamespacesResponse>, Status> {
        unimplemented!()
    }

    async fn delete_namespace(
        &self,
        _request: Request<encore::daemon::DeleteNamespaceRequest>,
    ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn dump_meta(
        &self,
        _request: Request<encore::daemon::DumpMetaRequest>,
    ) -> Result<Response<encore::daemon::DumpMetaResponse>, Status> {
        unimplemented!()
    }

    async fn telemetry(
        &self,
        _request: Request<encore::daemon::TelemetryConfig>,
    ) -> Result<Response<()>, Status> {
        unimplemented!()
    }

    async fn create_app(
        &self,
        _request: Request<encore::daemon::CreateAppRequest>,
    ) -> Result<Response<encore::daemon::CreateAppResponse>, Status> {
        unimplemented!()
    }
}