use super::error;
use super::hazard;

use utils::app_root::find_app_root;
use utils::app_config::AppConfig;
use utils::error::Result;
use std::path::Path;
use futures_util::StreamExt;
use grpc::encore::daemon::CheckRequest;

pub async fn check_command(codegen_debug: bool, parse_tests: bool) -> Result<()> {
    // 1. Find app root and rel path
    let (app_root, rel_path) = find_app_root()?;

    // 2. Handle SIGINT for cancellation
    let cancel_token = setup_signal_handler();

    // 3. Connect to daemon (gRPC)
    let mut client = connect_to_daemon().await?;

    // 4. Build request
    let request = CheckRequest {
        app_root: app_root.to_string_lossy().to_string(),
        working_dir: rel_path.to_string_lossy().to_string(),
        codegen_debug,
        parse_tests,
        environ: std::env::vars()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect(),
    };

    // 5. Send request and stream output
    let mut stream = client.check(request).await?.into_inner();
    while let Some(msg) = stream.next().await {
        handle_output(msg)?;
    }

    // 6. Exit with appropriate code
    let exit_code = 0;
    std::process::exit(exit_code);
}

// Dummy implementations for missing functions
fn setup_signal_handler() -> () {
    // TODO: Implement signal handling
}

async fn connect_to_daemon() -> Result<grpc::encore::daemon::daemon_client::DaemonClient<tonic::transport::Channel>> {
    // TODO: Use the correct address
    Ok(grpc::encore::daemon::daemon_client::DaemonClient::connect("http://[::1]:50051").await?)
}

fn handle_output(_msg: grpc::encore::daemon::CommandMessage) -> Result<()> {
    // TODO: Implement output handling
    Ok(())
}

pub fn bundle(entrypoint: &Path, outdir: &Path) -> Result<()> {
    let status = std::process::Command::new("tsbundler-encore")
        .arg("--bundle")
        .arg("--engine=node:21")
        .arg(format!("--outdir={}", outdir.display()))
        .arg(entrypoint)
        .status()?;
    
    if !status.success() {
        error!("Error: Bundling failed");
    }
    
    println!("Bundling succeeded!");
    Ok(())
}

pub fn run(watch: &bool, port: &Option<u16>) -> Result<()> {
    // let status = std::process::Command::new("tsbundler-encore")
    //     .arg("--bundle")
    //     .arg("--engine=node:21")
    //     .arg(format!("--outdir={}", outdir.display()))
    //     .arg(entrypoint)
    //     .status()?;
    // if !status.success() {
    //     return Err(anyhow::anyhow!("Bundling failed"));
    // }
    println!("Running succeeded!");
    Ok(())
}

/// Show the configuration file
pub fn hazard() -> Result<()> {
    // Generate, randomly, True or False
    let random_hazard: bool = hazard::generate_hazard()?;

    if random_hazard {
        println!("You got it right!");
    } else {
        println!("You got it wrong!");
    }

    Ok(())
}

/// Show the configuration file
pub fn config() -> Result<()> {
    let config = AppConfig::fetch()?;
    println!("{:#?}", config);

    Ok(())
}

/// Simulate an error
pub fn simulate_error() -> Result<()> {
    // Log this Error simulation
    info!("We are simulating an error");

    // Simulate an error
    error::simulate_error()?;

    // We should never get here...
    Ok(())
}
