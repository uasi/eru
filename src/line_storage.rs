use std::sync::Arc;

use item::Item;
use query::Query;

#[derive(Clone)]
pub struct LineStorage {
    lines: Vec<Arc<String>>,
}

impl LineStorage {
    pub fn new() -> Self {
        LineStorage {
            lines: Vec::new(),
        }
    }

    pub fn find(&self, query: &Query) -> Vec<Item> {
        self.lines.iter()
            .filter_map(|line| {
                match query.test(line) {
                    true  => Some(Item::new(line.clone())),
                    false => None,
                }
            })
            .collect()
    }

    pub fn put_chunk(&mut self, chunk: Vec<Arc<String>>) {
        self.lines.extend(chunk);
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }
}
