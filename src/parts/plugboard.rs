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
  pub fn add_connection(mut self, con1: usize, con2: usize) -> Self {
    assert_ne!(con1, con2);
    assert!(!self.connections.contains_key(&con1));
    assert!(!self.connections.contains_key(&con2));
    self.connections.insert(con1, con2);
    self.connections.insert(con2, con1);
    self
  }

  pub fn build(self) -> Plugboard {
    Plugboard {
      connections: self.connections,
    }
  }
}
