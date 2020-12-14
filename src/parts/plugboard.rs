use std::collections::HashMap;

use super::ALPHABET_LEN;

#[derive(Clone)]
pub struct Plugboard {
  connections: HashMap<usize, usize>,
}

impl Plugboard {
  pub fn new() -> PlugboardBuilder {
    PlugboardBuilder {
      connections: HashMap::with_capacity(ALPHABET_LEN),
    }
  }

  #[inline(always)]
  pub fn pass(&self, val: usize) -> usize {
    match self.connections.get(&val) {
      Some(&res) => res,
      None => val,
    }
  }
}

pub struct PlugboardBuilder {
  connections: HashMap<usize, usize>,
}

impl PlugboardBuilder {
  pub fn add_connection(mut self, connection: (usize, usize)) -> Self {
    assert_ne!(connection.0, connection.1);
    assert!(!self.connections.contains_key(&connection.0));
    assert!(!self.connections.contains_key(&connection.1));
    self.connections.insert(connection.0, connection.1);
    self.connections.insert(connection.1, connection.0);
    self
  }

  pub fn build(self) -> Plugboard {
    Plugboard {
      connections: self.connections,
    }
  }
}
