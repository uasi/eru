use std::cell::RefCell;
use std::cmp;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender};

use item::Item;
use key::Key;
use line_storage::LineStorage;
use query::QueryEditor;
use screen::Screen;
use screen_data::ScreenData;

pub struct State {
    highlighted_row: usize,
    item_index: usize,
    last_items: RefCell<Arc<Vec<Item>>>,
    last_query_string: RefCell<Option<String>>,
    line_storage: Arc<RwLock<LineStorage>>,
    query_editor: QueryEditor,
    screen: Screen,
}

pub enum StateInput {
    PutKey(Key),
    UpdateScreen,
}

pub enum StateReply {
    Complete(Option<Vec<Arc<String>>>),
}

impl State {
    pub fn new(line_storage: Arc<RwLock<LineStorage>>, screen: Screen) -> Self {
        State {
            highlighted_row: 0,
            item_index: 0,
            last_items: RefCell::new(Arc::new(Vec::new())),
            last_query_string: RefCell::new(None),
            line_storage: line_storage,
            query_editor: QueryEditor::new(),
            screen: screen,
        }
    }

    pub fn start(mut self, input_rx: Receiver<StateInput>, reply_tx: Sender<StateReply>) {
        loop {
            match input_rx.recv() {
                Ok(input) => {
                    let reply = self.process_input(input);
                    if let Some(reply) = reply {
                        reply_tx.send(reply).is_ok() || break;
                    }
                }
                Err(_) => break,
            }
        }
    }

    fn process_input(&mut self, input: StateInput) -> Option<StateReply> {
        use key::Key;
        use self::StateInput::*;
        use self::StateReply::*;
        match input {
            PutKey(Key::CtrlM) => {
                let sd = self.get_screen_data();
                return Some(Complete(sd.items.get(self.item_index+self.highlighted_row).and_then(|i| Some(vec![i.string.clone()]))));
            }
            PutKey(Key::CtrlN) => {
                self.move_highlight_forward();
                self.screen.update(self.get_screen_data());
            }
            PutKey(Key::CtrlP) => {
                self.move_highlight_backward();
                self.screen.update(self.get_screen_data());
            }
            PutKey(key) => {
                self.query_editor.put_key(key);
                let num_items = self.get_screen_data().items.len();
                self.highlighted_row = cmp::min(self.highlighted_row, cmp::max(num_items, 1) - 1);
                self.screen.update(self.get_screen_data());
            }
            UpdateScreen => {
                self.screen.update(self.get_screen_data());
            }
        }
        None
    }

    fn move_highlight_backward(&mut self) {
        let num_items = self.get_items().len();
        if num_items == 0 {
            self.highlighted_row = 0;
            return;
        }
        if self.highlighted_row == 0 {
            self.scroll_item_list_backward();
            return;
        }
        self.highlighted_row -= 1;
    }

    fn move_highlight_forward(&mut self) {
        let num_items = self.get_items().len();
        if num_items == 0 {
            self.highlighted_row = 0;
            return;
        }
        if self.highlighted_row >= num_items - 1 {
            return;
        }
        if self.highlighted_row >= self.screen.list_view_height() - 1 {
            self.scroll_item_list_forward();
            return;
        }
        self.highlighted_row += 1;
    }

    fn scroll_item_list_backward(&mut self) {
        if self.item_index > 0 {
            self.item_index -= 1;
        }
    }

    fn scroll_item_list_forward(&mut self) {
        let num_items = self.get_items().len();
        if self.item_index + self.screen.list_view_height() < num_items {
            self.item_index += 1;
        }
    }

    fn get_screen_data(&self) -> ScreenData {
        ScreenData {
            cursor_index: self.query_editor.cursor_position(),
            highlighted_row: self.highlighted_row,
            item_index: self.item_index,
            items: self.get_items(),
            query_string: Arc::new(self.query_editor.as_ref().to_string()),
            total_lines: self.line_storage.read().unwrap().len(),
        }
    }

    fn get_items(&self) -> Arc<Vec<Item>> {
        let last_qs = self.last_query_string.borrow();
        if last_qs.is_some() && last_qs.as_ref().unwrap() == self.query_editor.as_ref() {
            return self.last_items.borrow().clone();
        }
        drop(last_qs);
        let query = self.query_editor.query();
        let items = Arc::new(self.line_storage.read().unwrap().find(&query));
        *self.last_items.borrow_mut() = items.clone();
        *self.last_query_string.borrow_mut() = Some(self.query_editor.as_ref().to_string());
        items
    }
}
