use std::sync::Arc;

use item::Item;

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

    pub fn get_all(&self) -> Vec<Item> {
        self.lines.iter().map(|line| line.clone()).collect()
    }

    pub fn get_many_unchecked(&self, indices: Vec<usize>) -> Vec<Item> {
        indices.iter().map(|i| self.lines[*i].clone()).collect()
    }

    pub fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, Arc<String>> {
        self.lines.iter()
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn put_chunk(&mut self, chunk: Vec<Arc<String>>) {
        self.lines.extend(chunk);
    }
}
