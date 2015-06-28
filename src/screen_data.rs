use std::sync::Arc;

use item::Item;

#[derive(Clone)]
pub struct ScreenData {
    pub cursor_index: usize,
    pub highlighted_row: usize,
    pub item_list_len: usize,
    pub items: Vec<Item>,
    pub query_string: Arc<String>,
    pub marked_rows: Vec<usize>,
    pub total_lines: usize,
}
