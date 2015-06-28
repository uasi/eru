use std::boxed::Box;
use std::cmp;
use std::collections::BTreeSet;
use std::ops::Range;

pub struct ItemList {
    clipping_range_max_len: usize,
    clipping_range_start: usize,
    highlighted_row: Option<usize>,
    line_indices: Box<Indices>,
    marked_line_indices: BTreeSet<usize>,
}

impl ItemList {
    pub fn new(clipping_range_max_len: usize) -> Self {
        assert!(clipping_range_max_len > 0);
        ItemList {
            clipping_range_max_len: clipping_range_max_len,
            clipping_range_start: 0,
            highlighted_row: None,
            line_indices: Box::new(0..0),
            marked_line_indices: BTreeSet::new(),
        }
    }

    pub fn highlighted_row(&self) -> Option<usize> {
        self.highlighted_row
    }

    pub fn len(&self) -> usize {
        self.line_indices.count()
    }

    pub fn line_indices_in_clipping_range(&self) -> Vec<usize> {
        self.line_indices.boxed_iter()
            .skip(self.clipping_range_start)
            .take(self.clipping_range_end())
            .collect()
    }

    pub fn selected_line_indices(&self) -> Vec<usize> {
        match self.marked_line_indices.len() {
            0 => {
                match self.highlighted_row {
                    None => Vec::new(),
                    Some(row) => {
                        let i = self.clipping_range_start + row;
                        vec![self.line_indices.at(i)]
                    }
                }
            }
            _ => self.marked_line_indices.iter().cloned().collect(),
        }
    }

    pub fn marked_rows(&self) -> Vec<usize> {
        self.line_indices_in_clipping_range().iter()
            .enumerate()
            .filter_map(|(i, idx)| {
                if self.marked_line_indices.contains(idx) { Some(i) } else { None }
            })
            .collect()
    }

    pub fn toggle_mark(&mut self) {
        if let Some(row) = self.highlighted_row {
            let i = self.clipping_range_start + row;
            let line_index = self.line_indices.at(i);
            if !self.marked_line_indices.remove(&line_index) {
                self.marked_line_indices.insert(line_index);
            }
        }
    }

    pub fn set_line_indices(&mut self, line_indices: Vec<usize>) {
        self.set_line_indices_with_box::<Vec<usize>>(Box::new(line_indices));
    }

    pub fn set_line_index_range(&mut self, range: Range<usize>) {
        self.set_line_indices_with_box::<Range<usize>>(Box::new(range));
    }

    pub fn move_highlight_backward(&mut self) {
        if let Some(row) = self.highlighted_row {
            if row == 0 {
                self.scroll_backward();
                return;
            }
            self.highlighted_row = Some(row - 1);
        }
    }

    pub fn move_highlight_forward(&mut self) {
        if let Some(row) = self.highlighted_row {
            if Some(row) == self.max_row() {
                self.scroll_forward();
                return;
            }
            self.highlighted_row = Some(row + 1);
        }
    }

    pub fn scroll_backward(&mut self) {
        if self.clipping_range_start > 0 {
            self.clipping_range_start -= 1;
        }
    }

    pub fn scroll_forward(&mut self) {
        if self.clipping_range_end() < self.len() {
            self.clipping_range_start += 1;
        }
    }

    fn set_line_indices_with_box<T: Indices>(&mut self, line_indices: Box<Indices>) {
        self.line_indices = line_indices;
        let len = self.len();
        let range_end = self.clipping_range_end();
        if len < range_end {
            let diff = range_end - len;
            if diff > self.clipping_range_start {
                self.clipping_range_start = 0;
            } else {
                self.clipping_range_start -= diff;
            }
        }
        match (self.highlighted_row, self.max_row()) {
            (Some(row), Some(max_row)) if row > max_row => {
                self.highlighted_row = Some(max_row);
            }
            (None, Some(_)) => {
                self.highlighted_row = Some(0);
            }
            (_, None) => {
                self.highlighted_row = None;
            }
            (_, _) => { }
        }
        debug_assert!(
            (self.highlighted_row.is_some() && self.max_row().is_some()) ||
            (self.highlighted_row.is_none() && self.max_row().is_none()));
    }

    fn clipping_range_end(&self) -> usize {
        self.clipping_range_start + self.clipping_range_len()
    }

    fn clipping_range_len(&self) -> usize {
        cmp::min(self.len(), self.clipping_range_max_len)
    }

    fn max_row(&self) -> Option<usize> {
        match self.clipping_range_len() {
            0 => None,
            i => Some(i - 1),
        }
    }
}

pub trait Indices {
    fn at(&self, i: usize) -> usize;
    fn boxed_iter<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a>;
    fn count(&self) -> usize;
}

impl Indices for Range<usize> {
    fn at(&self, i: usize) -> usize {
        assert!(i < self.end);
        i
    }

    fn boxed_iter<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.clone())
    }

    fn count(&self) -> usize {
        self.clone().count()
    }
}

impl Indices for Vec<usize> {
    fn at(&self, i: usize) -> usize {
        unsafe { *self.get_unchecked(i) }
    }

    fn boxed_iter<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.iter().map(|i| *i))
    }

    fn count(&self) -> usize {
        self.len()
    }
}
