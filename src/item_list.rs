use std::cmp;

use item::Item;

pub struct ItemList {
    items: Vec<Item>,
    clipping_range_max_len: usize,
    clipping_range_start: usize,
    highlighted_row: usize,
}

impl ItemList {
    pub fn new(clipping_range_max_len: usize) -> Self {
        ItemList {
            items: Vec::new(),
            clipping_range_max_len: clipping_range_max_len,
            clipping_range_start: 0,
            highlighted_row: 0,
        }
    }

    pub fn highlighted_row(&self) -> usize {
        self.highlighted_row
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn items_in_clipping_range(&self) -> Vec<Item> {
        self.items.iter()
            .skip(self.clipping_range_start)
            .take(self.clipping_range_end())
            .map(|i| i.clone())
            .collect()
    }

    pub fn selected_items(&self) -> Vec<Item> {
        match self.items.len() {
            0 => Vec::new(),
            _ => vec![self.items[self.clipping_range_start + self.highlighted_row].clone()],
        }
    }

    pub fn set_items(&mut self, items: Vec<Item>) {
        self.items = items;
        let num_items = self.items.len();
        let range_end = self.clipping_range_end();
        if num_items < range_end {
            let diff = range_end - num_items;
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
        if self.clipping_range_end() < self.items.len() {
            self.clipping_range_start += 1;
        }
    }

    fn clipping_range_end(&self) -> usize {
        self.clipping_range_start + self.clipping_range_len()
    }

    fn clipping_range_len(&self) -> usize {
        cmp::min(self.items.len(), self.clipping_range_max_len)
    }

    fn max_row(&self) -> usize {
        match self.clipping_range_len() {
            0 => 0,
            i => i - 1,
        }
    }
}
