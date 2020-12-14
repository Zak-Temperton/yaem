use crate::u8_to_usize;

use super::ALPHABET_LEN;

#[allow(dead_code)]
pub enum ReflectorCode {
  ReflectorA,
  ReflectorB,
  ReflectorC,
  ReflectorBThin,
  ReflectorCThin,
  ETW,
  RUKW,
  KUKW,
}

impl ToString for ReflectorCode {
  fn to_string(&self) -> String {
    match self {
      ReflectorCode::ReflectorA => "ejmzalyxvbwfcrquontspikhgd".to_string(),
      ReflectorCode::ReflectorB => "yruhqsldpxngokmiebfzcwvjat".to_string(),
      ReflectorCode::ReflectorC => "fvpjiaoyedrzxwgctkuqsbnmhl".to_string(),
      ReflectorCode::ReflectorBThin => "enkqauywjicopblmdxzvfthrgs".to_string(),
      ReflectorCode::ReflectorCThin => "rdobjntkvehmlfcwzaxgyipsuq".to_string(),
      ReflectorCode::ETW => "abcdefghijklmnopqrstuvwxyz".to_string(),
      ReflectorCode::RUKW => "qyhognecvpuztfdjaxwmkisrbl".to_string(),
      ReflectorCode::KUKW => "imetcgfraysqbzxwlhkdvupojn".to_string(),
    }
  }
}
#[derive(Clone)]
pub struct Reflector {
  connections: [usize; ALPHABET_LEN],
}

impl Reflector {
  pub fn new(code: ReflectorCode) -> Self {
    Self::from_string(code.to_string())
  }

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
