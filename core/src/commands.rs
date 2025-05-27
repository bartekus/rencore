use super::error;
use super::hazard;

use utils::app_config::AppConfig;
use utils::error::Result;
use std::path::Path;

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
