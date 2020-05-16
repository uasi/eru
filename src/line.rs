pub struct Line {
    bytes: Vec<u8>,
    chars: Vec<char>,
    lossy_string: Option<String>,
}

impl Line {
    pub fn new(bytes: Vec<u8>) -> Line {
        match String::from_utf8(bytes) {
            Ok(string) => {
                let chars = string.chars().collect();
                Line {
                    bytes: string.into_bytes(),
                    chars,
                    lossy_string: None,
                }
            }
            Err(error) => {
                let bytes = error.into_bytes();
                let lossy = String::from_utf8_lossy(&bytes).into_owned();
                let chars = lossy.chars().collect();
                Line {
                    bytes,
                    chars,
                    lossy_string: Some(lossy),
                }
            }
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_chars(&self) -> &[char] {
        &self.chars
    }

    pub fn as_str(&self) -> &str {
        match self.lossy_string {
            Some(ref s) => s,
            None => std::str::from_utf8(&self.bytes).unwrap(),
        }
    }
}
