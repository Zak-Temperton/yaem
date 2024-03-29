use super::{reflector::Reflector, u8_to_usize, ALPHABET_LEN};
#[derive(Clone)]
pub struct Rotor {
    connections: [(usize, usize); ALPHABET_LEN],
    rotation: usize,
}

impl Rotor {
    #[inline(always)]
    pub fn new(code: RotorCode) -> Self {
        Self::from_string(code.to_string())
    }

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

    #[inline(always)]
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

#[derive(Clone)]
pub struct Rotors {
    rotors: (Rotor, Rotor, Rotor),
    reflector: Reflector,
}

impl Rotors {
    #[inline(always)]
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

#[allow(dead_code)]
pub enum RotorCode {
    CI,
    CII,
    CIII,
    RI,
    RII,
    RIII,
    ETW,
    KI,
    KII,
    KIII,
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII,
    VIII,
    Beta,
    Gamma,
}

impl ToString for RotorCode {
    fn to_string(&self) -> String {
        match self {
            RotorCode::CI => "dmtwsilruyqnkfejcazbpgxohv".to_string(),
            RotorCode::CII => "hqzgpjtmoblncifdyawveusrkx".to_string(),
            RotorCode::CIII => "uqntlszfmrehdpxkibvygjcwoa".to_string(),
            RotorCode::RI => "jgdqoxuscamifrvtpnewkblzyh".to_string(),
            RotorCode::RII => "ntzpsfbokmwrcjdivlaeyuxhgq".to_string(),
            RotorCode::RIII => "jviubhtcdyakeqzposgxnrmwfl".to_string(),
            RotorCode::ETW => "qwertzuioasdfghjkpyxcvbnml".to_string(),
            RotorCode::KI => "pezuohxscvfmtbglrinqjwaydk".to_string(),
            RotorCode::KII => "zouesydkfwpciqxhmvblgnjrat".to_string(),
            RotorCode::KIII => "ehrvxgaobqusimzflynwktpdjc".to_string(),
            RotorCode::I => "ekmflgdqvzntowyhxuspaibrcj".to_string(),
            RotorCode::II => "ajdksiruxblhwtmcqgznpyfvoe".to_string(),
            RotorCode::III => "bdfhjlcprtxvznyeiwgakmusqo".to_string(),
            RotorCode::IV => "esovpzjayquirhxlnftgkdcmwb".to_string(),
            RotorCode::V => "vzbrgityupsdnhlxawmjqofeck".to_string(),
            RotorCode::VI => "jpgvoumfyqbenhzrdkasxlictw".to_string(),
            RotorCode::VII => "nzjhgrcxmyswboufaivlpekqdt".to_string(),
            RotorCode::VIII => "fkqhtlxocbjspdzramewniuygv".to_string(),
            RotorCode::Beta => "leyjvcnixwpbqmdrtakzgfuhos".to_string(),
            RotorCode::Gamma => "fsokanuerhmbtiycwlqpzxvgjd".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parts::{reflector::Reflector, ALPHABET_LEN};

    use super::{Rotor, RotorCode, Rotors};

    #[test]
    fn rotor() {
        let r = Rotor::new(RotorCode::I).with_rotation(2);
        for i in 0..ALPHABET_LEN {
            assert_eq!(i, r.pass_back(r.pass_forward(i)));
        }
    }

    #[test]
    fn rotor2() {
        let r = Rotor::from_string("ekmflgdqvzntowyhxuspaibrcj".to_string()).with_rotation(4);
        for i in 0..ALPHABET_LEN {
            print!("{:2},", i);
            print!("{:2},", r.pass_forward(i),);
            println!("{:2}", r.pass_back(r.pass_forward(i)));
            assert_eq!(i, r.pass_back(r.pass_forward(i)));
        }
    }

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
