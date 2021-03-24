mod enigma_machine;
mod parts;

use std::env;

use enigma_machine::EnigmaMachine;
use parts::{
  reflector::{Reflector, ReflectorCode},
  rotor::{Rotor, RotorCode, Rotors},
  u8_to_usize, usize_to_char, Plugboard, ALPHABET_LEN,
};

fn main() {
  let mut emach_1 = {
    let rotor1 = Rotor::new(RotorCode::I).with_rotation(5);
    let rotor2 = Rotor::new(RotorCode::II).with_rotation(1);
    let rotor3 = Rotor::new(RotorCode::III).with_rotation(24);
    let reflector = Reflector::new(ReflectorCode::ReflectorA);
    let rotors = Rotors::new(rotor1, rotor2, rotor3, reflector);
    let plugboard = Plugboard::new()
      .add_connection(1, 3)
      .add_connection(25, 7)
      .add_connection(2, 4)
      .add_connection_from_chars('f', 'j')
      .build();
    EnigmaMachine::new(rotors, plugboard)
  };
  let mut emach_2 = emach_1.clone();

  let args = env::args();
  if args.len() == 1 {
    println!("Needs to have an additional argument of charachters a..=z");
    return;
  }

  let input = args.skip(1).next().unwrap(); //skip first arg as it is the program name
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
      i == decoded,
    );
  }
  println!("\nEncoded: {}", result);
}
