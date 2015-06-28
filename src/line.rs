pub struct Line(pub Vec<u8>);

impl Line {
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}

impl AsRef<[u8]> for Line {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}
