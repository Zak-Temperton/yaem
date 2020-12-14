use crate::parts::{Plugboard, Rotors};

#[derive(Clone)]
pub struct EnigmaMachine {
  rotors: Rotors,
  plugboard: Plugboard,
}

impl EnigmaMachine {
  pub fn new(rotors: Rotors, plugboard: Plugboard) -> Self {
    EnigmaMachine { rotors, plugboard }
  }

  pub fn pass(&mut self, val: usize) -> usize {
    self
      .plugboard
      .pass(self.rotors.pass(self.plugboard.pass(val)))
  }
}
