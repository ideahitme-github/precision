use precision_proto::precision_server::{Precision, PrecisionServer};
use precision_proto::{CreateStreamRequest, CreateStreamResponse};
use tonic::{Request, Response, Status};

mod precision_proto {
  tonic::include_proto!("precision");
}

#[derive(Debug, Default)]
struct PrecisionImpl {}

#[tonic::async_trait]
impl Precision for PrecisionImpl {
  async fn create_stream(
    &self,
    _: Request<CreateStreamRequest>,
  ) -> Result<Response<CreateStreamResponse>, Status> {
    let reply = CreateStreamResponse {};
    Ok(Response::new(reply))
  }
}

pub fn new() -> PrecisionServer<impl Precision> {
  PrecisionServer::new(PrecisionImpl::default())
}
