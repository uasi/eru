pub struct Pattern {
    string: String,
}

impl Pattern {
    pub fn test(&self, haystack: &str) -> bool {
        haystack.contains(&self.string)
    }
}

pub fn patterns_from_str(s: &str) -> Vec<Pattern> {
    s.split(" ")
        .map(|t| Pattern { string: t.to_string() })
        .collect()
}
