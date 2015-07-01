use std::ascii::AsciiExt;

pub struct Pattern {
    chars: Vec<char>,
}

impl Pattern {
    pub fn test(&self, haystack: &[char]) -> bool {
        test_fuzzy_ignorecase(haystack, &self.chars)
    }
}

pub fn patterns_from_str(s: &str) -> Vec<Pattern> {
    s.split_whitespace()
        .map(|t| Pattern { chars: t.chars().collect() })
        .collect()
}

fn test_fuzzy_ignorecase(haystack: &[char], needle: &[char]) -> bool {
    debug_assert!(needle.len() > 0);
    let mut nidx = 0;
    for ch in haystack.iter() {
        let ch = ch.to_ascii_lowercase();
        if ch == needle[nidx] {
            nidx += 1;
            if nidx == needle.len() {
                return true;
            }
        }
    }
    false
}
