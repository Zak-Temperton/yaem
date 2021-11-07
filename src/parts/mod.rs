mod plugboard;
pub use plugboard::Plugboard;

pub mod reflector;
pub mod rotor;

pub const ALPHABET_LEN: usize = 26;

#[inline(always)]
pub fn usize_to_char(num: usize) -> char {
    (num + 'a' as usize) as u8 as char
}

#[inline(always)]
pub fn u8_to_usize(num: u8) -> usize {
    (num - 'a' as u8) as usize
}
