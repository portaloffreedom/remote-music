extern crate crypto;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

//#[derive(Clone)]
pub struct Auth {
    hex: String,
}

impl Auth {

    pub fn new(hex: String) -> Auth {
        Auth { hex: hex }
    }

    pub fn new_empty() -> Auth {
        Auth { hex: "".to_string() }
    }

    pub fn set_hex(&mut self, hex: String) {
        self.hex = hex;
    }

    pub fn authenticate(&self, data: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.input_str(data);
        let hex = hasher.result_str();

        hex == self.hex
    }
}
