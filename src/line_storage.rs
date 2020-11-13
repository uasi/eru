use std::sync::Arc;

use crate::item::Item;
use crate::line::Line;

#[derive(Clone)]
pub struct LineStorage {
    lines: Vec<Arc<Line>>,
}

impl LineStorage {
    pub fn new() -> Self {
        LineStorage { lines: Vec::new() }
    }

    pub fn get_many_unchecked(&self, indices: Vec<usize>) -> Vec<Item> {
        indices.iter().map(|i| self.lines[*i].clone()).collect()
    }

    pub fn iter<'a>(&'a self) -> ::std::slice::Iter<'a, Arc<Line>> {
        self.lines.iter()
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn put_chunk(&mut self, chunk: Vec<Arc<Line>>) {
        self.lines.extend(chunk);
    }
}
