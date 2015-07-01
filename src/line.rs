use self::Value::*;

enum Value {
    Lossless(String),
    Lossy(String, Vec<u8>),
}

pub struct Line {
    value: Value,
}

impl Line {
    pub fn new(bytes: Vec<u8>) -> Line {
        match String::from_utf8(bytes) {
            Ok(string) => {
                Line { value: Lossless(string) }
            }
            Err(error) => {
                let bytes = error.into_bytes();
                let string = String::from_utf8_lossy(&bytes).to_string();
                Line { value: Lossy(string, bytes) }
            }
        }
    }

    pub fn as_lossy_str(&self) -> &str {
        match self.value {
            Lossless(ref string) => string.as_ref(),
            Lossy(ref string, _) => string.as_ref(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self.value {
            Lossless(ref string) => string.as_bytes(),
            Lossy(_, ref bytes) => bytes,
        }
    }
}
