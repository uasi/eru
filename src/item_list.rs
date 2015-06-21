use std::boxed::Box;
use std::cmp;
use std::ops::Range;

pub struct ItemList {
    clipping_range_max_len: usize,
    clipping_range_start: usize,
    highlighted_row: usize,
    line_indices: Box<Indices>,
}

impl ItemList {
    pub fn new(clipping_range_max_len: usize) -> Self {
        ItemList {
            clipping_range_max_len: clipping_range_max_len,
            clipping_range_start: 0,
            highlighted_row: 0,
            line_indices: Box::new(0..0),
        }
    }

    pub fn highlighted_row(&self) -> usize {
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
        match self.len() {
            0 => Vec::new(),
            _ => vec![self.clipping_range_start + self.highlighted_row],
        }
    }

    pub fn set_line_indices(&mut self, line_indices: Vec<usize>) {
        self.set_line_indices_with_box::<Vec<usize>>(Box::new(line_indices));
    }

    pub fn set_line_index_range(&mut self, range: Range<usize>) {
        self.set_line_indices_with_box::<Range<usize>>(Box::new(range));
    }

    pub fn move_highlight_backward(&mut self) {
        if self.highlighted_row == 0 {
            self.scroll_backward();
            return;
        }
        self.highlighted_row -= 1;
    }

    pub fn move_highlight_forward(&mut self) {
        if self.highlighted_row == self.max_row() {
            self.scroll_forward();
            return;
        }
        self.highlighted_row += 1;
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
        let max_row = self.max_row();
        if self.highlighted_row > max_row {
            self.highlighted_row = max_row;
        }
    }


    fn clipping_range_end(&self) -> usize {
        self.clipping_range_start + self.clipping_range_len()
    }

    fn clipping_range_len(&self) -> usize {
        cmp::min(self.len(), self.clipping_range_max_len)
    }

    fn max_row(&self) -> usize {
        match self.clipping_range_len() {
            0 => 0,
            i => i - 1,
        }
    }
}

pub trait Indices {
    fn boxed_iter<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a>;
    fn count(&self) -> usize;
}

impl Indices for Range<usize> {
    fn boxed_iter<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.clone())
    }

    fn count(&self) -> usize {
        self.clone().count()
    }
}

impl Indices for Vec<usize> {
    fn boxed_iter<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.iter().map(|i| *i))
    }

    fn count(&self) -> usize {
        self.len()
    }
}
