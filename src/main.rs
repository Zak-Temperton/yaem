use std::env;

use enigma_machine::EnigmaMachine;
use parts::{Plugboard, Reflector, Rotor, Rotors, ALPHABET_LEN};

mod enigma_machine;
mod parts;
fn main() {
  let rotor1 = Rotor::from_string("ekmflgdqvzntowyhxuspaibrcj".to_string()).with_rotation(5);
  let rotor2 = Rotor::from_string("ajdksiruxblhwtmcqgznpyfvoe".to_string()).with_rotation(1);
  let rotor3 = Rotor::from_string("bdfhjlcprtxvznyeiwgakmusqo".to_string()).with_rotation(24);
  let reflector = Reflector::from_string("ejmzalyxvbwfcrquontspikhgd".to_string());
  let rotors = Rotors::new(rotor1, rotor2, rotor3, reflector);
  let plugboard = Plugboard::new()
    .add_connection(1, 3)
    .add_connection(25, 7)
    .add_connection(2, 4)
    .build();
  let mut emach_1 = EnigmaMachine::new(rotors, plugboard);
  let mut emach_2 = emach_1.clone();

  let input = env::args().skip(1).next().unwrap(); //skip ffirst as it is the name of the program
  let mut result = String::with_capacity(input.len());
  for i in input.bytes() {
    let i = u8_to_usize(i);
    assert!(i < ALPHABET_LEN);

    let encoded = emach_1.pass(i);
    let decoded = emach_2.pass(encoded);

    result.push(usize_to_char(encoded));
    println!(
      "input: '{}', encoded: '{}', decoded: '{}', correct: {}",
      usize_to_char(i),
      usize_to_char(encoded),
      usize_to_char(decoded),
      i == decoded
    );
  }
  println!("\nEncoded: {}", result.as_str());
}

#[inline(always)]
fn usize_to_char(num: usize) -> char {
  (num + 'a' as usize) as u8 as char
}

#[inline(always)]
fn u8_to_usize(num: u8) -> usize {
  (num - 'a' as u8) as usize
}
