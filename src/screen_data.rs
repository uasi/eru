use std::sync::Arc;

use item::Item;

#[derive(Clone)]
pub struct ScreenData {
    pub cursor_index: usize,
    pub highlighted_row: usize,
    pub item_index: usize,
    pub items: Arc<Vec<Item>>,
    pub query_string: Arc<String>,
    pub total_lines: usize,
}
