use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::Mutex;
use tonic::transport::Server;
mod handler;
mod user_stream;

pub async fn start_server(addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
  Server::builder()
    .add_service(handler::new(Arc::new(Mutex::new(
      user_stream::new_memory_stream_collection(),
    ))))
    .serve(addr)
    .await?;
  Ok(())
}
