use crate::parts::{rotor::Rotors, Plugboard};

#[derive(Clone)]
pub struct EnigmaMachine {
  rotors: Rotors,
  plugboard: Plugboard,
}

impl EnigmaMachine {
  #[inline(always)]
  pub fn new(rotors: Rotors, plugboard: Plugboard) -> Self {
    EnigmaMachine { rotors, plugboard }
  }

  #[inline(always)]
  pub fn pass(&mut self, val: usize) -> usize {
    self
      .plugboard
      .pass(self.rotors.pass(self.plugboard.pass(val)))
  }
}
