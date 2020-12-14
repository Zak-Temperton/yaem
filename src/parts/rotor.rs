use crate::u8_to_usize;

use super::ALPHABET_LEN;
#[derive(Clone)]
pub struct Rotor {
  connections: [(usize, usize); ALPHABET_LEN],
  rotation: usize,
}

impl Rotor {
  pub fn from_string(string: String) -> Self {
    assert_eq!(string.len(), ALPHABET_LEN);
    let mut connections = [(0, 0); ALPHABET_LEN];
    for (i, c) in string.bytes().enumerate() {
      let c = u8_to_usize(c);
      connections[i].0 = c;
      connections[c].1 = i;
    }
    Rotor {
      connections,
      rotation: 0,
    }
  }

  #[inline(always)]
  pub fn with_rotation(mut self, rotation: usize) -> Self {
    self.rotation = rotation;
    self
  }

  #[inline(always)]
  pub fn pass_forward(&self, val: usize) -> usize {
    (self.connections[(val + self.rotation) % ALPHABET_LEN].0 + self.rotation) % ALPHABET_LEN
  }

  pub fn pass_back(&self, val: usize) -> usize {
    let i = Self::loop_val(val as i32 - self.rotation as i32);
    Self::loop_val(self.connections[i].1 as i32 - self.rotation as i32)
  }

  #[inline(always)]
  pub fn rotate(&mut self) -> bool {
    self.rotation += 1;
    if self.rotation == ALPHABET_LEN {
      self.rotation = 0;
      true
    } else {
      false
    }
  }

  #[inline(always)]
  fn loop_val(val: i32) -> usize {
    if val < 0 {
      (ALPHABET_LEN as i32 + val) as usize
    } else {
      val as usize
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::parts::ALPHABET_LEN;

  use super::Rotor;

  #[test]
  fn rotor() {
    let r = Rotor::from_string("abcdefghijklmnopqrstuvwxyz".to_string()).with_rotation(2);
    for i in 0..ALPHABET_LEN {
      assert_eq!(i, r.pass_back(r.pass_forward(i)));
    }
  }

  #[test]
  fn rotor2() {
    let r = Rotor::from_string("ekmflgdqvzntowyhxuspaibrcj".to_string()).with_rotation(4);
    for i in 0..ALPHABET_LEN {
      print!("{},", i);
      print!("{},", r.pass_forward(i),);
      println!("{}", r.pass_back(r.pass_forward(i)));
      assert_eq!(i, r.pass_back(r.pass_forward(i)));
    }
  }
}
