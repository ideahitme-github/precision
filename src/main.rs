use tonic::transport::Server;

mod handler;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;

    Server::builder()
        .add_service(handler::handler::new())
        .serve(addr)
        .await?;
    Ok(())
}
