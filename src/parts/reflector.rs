use crate::u8_to_usize;

use super::ALPHABET_LEN;

#[derive(Clone)]
pub struct Reflector {
  connections: [usize; ALPHABET_LEN],
}

impl Reflector {
  pub fn from_string(string: String) -> Self {
    assert_eq!(string.len(), ALPHABET_LEN);
    let mut connections = [0; ALPHABET_LEN];
    for (i, c) in string.bytes().enumerate() {
      connections[i] = u8_to_usize(c);
    }
    Reflector { connections }
  }

  #[inline(always)]
  pub fn pass(&self, val: usize) -> usize {
    self.connections[val]
  }
}
