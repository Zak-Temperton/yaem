use super::{reflector::Reflector, rotor::Rotor};

#[derive(Clone)]
pub struct Rotors {
  rotors: (Rotor, Rotor, Rotor),
  reflector: Reflector,
}

impl Rotors {
  pub fn new(rotor1: Rotor, rotor2: Rotor, rotor3: Rotor, reflector: Reflector) -> Self {
    Rotors {
      rotors: (rotor1, rotor2, rotor3),
      reflector,
    }
  }
  pub fn pass(&mut self, mut val: usize) -> usize {
    val = self
      .rotors
      .2
      .pass_forward(self.rotors.1.pass_forward(self.rotors.0.pass_forward(val)));
    val = self.reflector.pass(val);
    val = self
      .rotors
      .0
      .pass_back(self.rotors.1.pass_back(self.rotors.2.pass_back(val)));

    if self.rotors.0.rotate() {
      if self.rotors.1.rotate() {
        self.rotors.2.rotate();
      }
    }

    val
  }
}

#[cfg(test)]
mod tests {
  use crate::parts::{reflector::Reflector, rotor::Rotor};

  use super::Rotors;

  #[test]
  fn rotors() {
    let r1 = Rotor::from_string("ekmflgdqvzntowyhxuspaibrcj".to_string());
    let r2 = Rotor::from_string("ajdksiruxblhwtmcqgznpyfvoe".to_string());
    let r3 = Rotor::from_string("bdfhjlcprtxvznyeiwgakmusqo".to_string());
    let reflector = Reflector::from_string("ejmzalyxvbwfcrquontspikhgd".to_string());
    let mut rs1 = Rotors::new(r1.clone(), r2.clone(), r3.clone(), reflector.clone());
    let mut rs2 = Rotors::new(r1, r2, r3, reflector);
    let i = 0;
    let res = rs1.pass(i);
    assert_eq!(i, rs2.pass(res));
  }
}
