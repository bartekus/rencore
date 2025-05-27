use tonic::{transport::Server, Request, Response, Status};
use encore::daemon::daemon_server::{Daemon, DaemonServer};
use encore::daemon::{CheckRequest, CommandMessage};

pub mod encore {
    pub mod daemon {
        tonic::include_proto!("encore.daemon");
    }
}

#[derive(Default)]
pub struct MyDaemon {}

#[tonic::async_trait]
impl Daemon for MyDaemon {
    type CheckStream = tokio_stream::wrappers::ReceiverStream<Result<CommandMessage, Status>>;

    async fn check(
        &self,
        request: Request<CheckRequest>,
    ) -> Result<Response<Self::CheckStream>, Status> {
        println!("Check called with: {:?}", request);

        // Example: send a single CommandMessage and close the stream
        let (tx, rx) = tokio::sync::mpsc::channel(4);
        let msg = CommandMessage {
            msg: Some(encore::daemon::command_message::Msg::Output(
                encore::daemon::CommandOutput {
                    stdout: b"Check completed successfully\n".to_vec(),
                    stderr: vec![],
                },
            )),
        };
        tx.send(Ok(msg)).await.unwrap();

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(rx)))
    }

    // Implement other methods as needed...
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let daemon = MyDaemon::default();

    println!("DaemonServer listening on {addr}");

    Server::builder()
        .add_service(DaemonServer::new(daemon))
        .serve(addr)
        .await?;

    Ok(())
}