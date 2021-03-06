use unicode_width::UnicodeWidthStr;

use crate::key::Key;
use crate::pattern::{self, Pattern};

pub struct Query {
    patterns: Vec<Pattern>,
    string: String,
}

impl Query {
    fn new(string: String) -> Self {
        Query {
            patterns: pattern::patterns_from_str(&string),
            string,
        }
    }

    pub fn test(&self, haystack: &[char]) -> bool {
        self.patterns.iter().all(|p| p.test(haystack))
    }
}

impl AsRef<str> for Query {
    fn as_ref(&self) -> &str {
        self.string.as_ref()
    }
}

pub struct QueryEditor {
    cursor_position: usize,
    string: String,
}

impl QueryEditor {
    pub fn new<S: Into<String>>(string: S, is_cjk: bool) -> QueryEditor {
        let string = string.into();
        let width = if is_cjk {
            <str as UnicodeWidthStr>::width_cjk(&string)
        } else {
            <str as UnicodeWidthStr>::width(&string)
        };
        QueryEditor {
            cursor_position: width,
            string,
        }
    }

    pub fn put_key(&mut self, key: Key) {
        use crate::key::Key::*;
        match key {
            CtrlA => {
                self.cursor_position = 0;
            }
            CtrlB => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            CtrlD => {
                if self.cursor_position < self.string.len() {
                    let cursor = self.cursor_position;
                    self.string.remove(cursor);
                }
            }
            CtrlE => {
                self.cursor_position = self.string.len();
            }
            CtrlF => {
                if self.cursor_position < self.string.len() {
                    self.cursor_position += 1;
                }
            }
            CtrlH | Del => {
                if self.cursor_position > 0 {
                    let cursor = self.cursor_position;
                    self.string.remove(cursor - 1);
                    self.cursor_position -= 1;
                }
            }
            CtrlK => {
                let cursor = self.cursor_position;
                self.string.truncate(cursor);
            }
            CtrlW => {
                let cursor = self.cursor_position;
                let word_end = self.string[0..cursor].rfind(|ch| ch != ' ').unwrap_or(0);
                let word_start = self.string[0..word_end]
                    .rfind(' ')
                    .map(|i| i + 1)
                    .unwrap_or(0);
                self.string.truncate(word_start);
                self.cursor_position = word_start;
            }
            Char(ch) => {
                let cursor = self.cursor_position;
                self.string.insert(cursor, ch);
                self.cursor_position += 1;
            }
            _ => {}
        }
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_position as usize
    }

    pub fn query(&self) -> Query {
        Query::new(self.string.clone())
    }
}

impl AsRef<str> for QueryEditor {
    fn as_ref(&self) -> &str {
        self.string.as_ref()
    }
}
