use crate::user_stream::Stream;
use crate::user_stream::StreamCollection;
use precision_proto::precision_server::{Precision, PrecisionServer};
use precision_proto::{CreateStreamRequest, CreateStreamResponse};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

mod precision_proto {
  tonic::include_proto!("precision");
}

struct PrecisionImpl {
  streams: Arc<Mutex<dyn StreamCollection>>,
}

impl PrecisionImpl {
  pub fn new(streams: Arc<Mutex<dyn StreamCollection>>) -> Self {
    return PrecisionImpl { streams: streams };
  }
}

#[tonic::async_trait]
impl Precision for PrecisionImpl {
  async fn create_stream(
    &self,
    request: Request<CreateStreamRequest>,
  ) -> Result<Response<CreateStreamResponse>, Status> {
    let new_stream = Stream::new(request.into_inner().user_id);
    let mut streams = self.streams.lock().map_err(|_| -> Status {
      Status::internal("failed to acquire lock of streams collection")
    })?;
    streams
      .push(new_stream)
      .map_err(|_| -> Status { Status::invalid_argument("specified stream already exists!") })?;

    Ok(Response::new(CreateStreamResponse {}))
  }
}

pub fn new(streams: Arc<Mutex<dyn StreamCollection>>) -> PrecisionServer<impl Precision> {
  PrecisionServer::new(PrecisionImpl::new(streams))
}
