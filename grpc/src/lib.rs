mod client;
mod server;

use tonic::{Request, Response, Status};
use server::MyDaemon;
use server::encore::daemon::{CheckRequest, CommandMessage};
use server::encore::daemon::daemon_server::Daemon;

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    // does nothing

    Ok(())
}
