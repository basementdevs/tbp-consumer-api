use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
pub struct ThrottleState {
  pub last_request: HashMap<(String, String), Instant>,
}

impl ThrottleState {
  pub fn new() -> Self {
    Self {
      last_request: HashMap::new(),
    }
  }
}

impl Default for ThrottleState {
  fn default() -> Self {
    Self::new()
  }
}
