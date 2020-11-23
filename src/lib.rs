use std::net::SocketAddr;
use tonic::transport::Server;
mod handler;

pub async fn start_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
  Server::builder()
    .add_service(handler::new())
    .serve(addr)
    .await?;
  Ok(())
}
