pub struct Pattern {
    string: String,
}

impl Pattern {
    pub fn test(&self, haystack: &[u8]) -> bool {
        contains(haystack, self.string.as_ref())
    }
}

pub fn patterns_from_str(s: &str) -> Vec<Pattern> {
    s.split(" ")
        .map(|t| Pattern { string: t.to_string() })
        .collect()
}

// FIXME: make less naive
fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() == 0 {
        return true;
    } else if needle.len() == 1 {
        return haystack.contains(&needle[0]);
    }
    for window in haystack.windows(needle.len()) {
        if window == needle {
            return true;
        }
    }
    false
}
