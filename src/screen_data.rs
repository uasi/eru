use std::sync::Arc;

use crate::item::Item;

#[derive(Clone)]
pub struct ScreenData {
    pub cursor_index: usize,
    pub highlighted_row: Option<usize>,
    pub is_cjk: bool,
    pub item_list_len: usize,
    pub items: Vec<Item>,
    pub marked_rows: Vec<usize>,
    pub query_string: Arc<String>,
    pub status_message: Option<String>,
    pub total_lines: usize,
}
