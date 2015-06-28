pub struct Line(pub String);

impl AsRef<str> for Line {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
