use std::collections::HashMap;

use super::{u8_to_usize, ALPHABET_LEN};

#[derive(Clone)]
pub struct Plugboard {
    connections: HashMap<usize, usize>, //due to known size of alphabet a arrar/vec could be used
}

impl Plugboard {
    #[inline(always)]
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

//Everything is inlined as they are not likely to be called often
impl PlugboardBuilder {
    // #[inline(always)]
    // pub fn add_connection(mut self, con1: usize, con2: usize) -> Self {
    //     assert_ne!(con1, con2);
    //     assert!(!self.connections.contains_key(&con1));
    //     assert!(!self.connections.contains_key(&con2));
    //     self.connections.insert(con1, con2);
    //     self.connections.insert(con2, con1);
    //     self
    // }

    // #[inline(always)]
    // pub fn add_connection_from_chars(self, con1: char, con2: char) -> Self {
    //     self.add_connection(u8_to_usize(con1 as u8), u8_to_usize(con2 as u8))
    // }

    #[inline(always)]
    pub fn build(self) -> Plugboard {
        Plugboard {
            connections: self.connections,
        }
    }
}

pub trait AddConnection<T> {
    fn add_connection(self, con1: T, con2: T) -> Self;
}

impl AddConnection<usize> for PlugboardBuilder {
    #[inline(always)]
    fn add_connection(mut self, con1: usize, con2: usize) -> Self {
        assert_ne!(con1, con2);
        assert!(!self.connections.contains_key(&con1));
        assert!(!self.connections.contains_key(&con2));
        self.connections.insert(con1, con2);
        self.connections.insert(con2, con1);
        self
    }
}

impl AddConnection<char> for PlugboardBuilder {
    #[inline(always)]
    fn add_connection(self, con1: char, con2: char) -> Self {
        self.add_connection(u8_to_usize(con1 as u8), u8_to_usize(con2 as u8))
    }
}
