use crate::user_stream::stream::Stream;
use std::collections::HashMap;
use std::fmt;

pub trait StreamCollection: Sync + Send {
  fn push(&mut self, stream: Stream) -> Result<(), StreamAlreadyExistsError>;
  fn find_stream(&self, stream_id: String) -> Result<&Stream, StreamNotFoundError>;
}

#[derive(Debug, Clone)]
pub struct StreamAlreadyExistsError {}

impl fmt::Display for StreamAlreadyExistsError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "stream not found!")
  }
}

#[derive(Debug, Clone)]
pub struct StreamNotFoundError {}

impl fmt::Display for StreamNotFoundError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "stream not found!")
  }
}

#[derive(Default)]
struct MemoryStreamCollection {
  streams: HashMap<String, Stream>,
}

impl StreamCollection for MemoryStreamCollection {
  fn push(&mut self, stream: Stream) -> Result<(), StreamAlreadyExistsError> {
    if self.streams.contains_key(&stream.id()) {
      return Err(StreamAlreadyExistsError {});
    }
    self.streams.insert(stream.id(), stream);
    // write to DB
    Ok(())
  }
  fn find_stream(&self, stream_id: String) -> Result<&Stream, StreamNotFoundError> {
    self.streams.get(&stream_id).ok_or(StreamNotFoundError {})
  }
}
pub fn new_memory_stream_collection() -> impl StreamCollection {
  MemoryStreamCollection::default()
}
