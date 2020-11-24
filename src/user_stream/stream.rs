pub struct Stream {
  user_id: String,
}

impl Stream {
  pub fn new(user_id: String) -> Self {
    Stream { user_id }
  }

  pub fn id(&self) -> String {
    return self.user_id.clone();
  }
}
