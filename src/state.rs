use std::cmp;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use item::Item;
use key::Key;
use query::{Query, QueryEditor};
use screen::Screen;
use screen_data::ScreenData;

pub struct State {
    highlighted_row: usize,
    item_index: usize,
    last_screen_data: Option<ScreenData>,
    line_storage: LineStorage,
    query_editor: QueryEditor,
    screen: Screen,
}

pub enum StateInput {
    PutChunk(Vec<Arc<String>>),
    PutKey(Key),
}

pub enum StateReply {
    Complete(Option<Vec<Arc<String>>>),
}

impl State {
    pub fn new(screen: Screen) -> Self {
        State {
            highlighted_row: 0,
            item_index: 0,
            last_screen_data: None,
            line_storage: LineStorage::new(),
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
                let sd = self.get_screen_data();
                self.screen.update(sd);
            }
            PutKey(Key::CtrlP) => {
                self.move_highlight_backward();
                let sd = self.get_screen_data();
                self.screen.update(sd);
            }
            PutKey(key) => {
                self.query_editor.put_key(key);
                let num_items = self.get_screen_data().items.len();
                self.highlighted_row = cmp::min(self.highlighted_row, cmp::max(num_items, 1) - 1);
                let sd = self.get_screen_data();
                self.screen.update(sd);
            }
            PutChunk(chunk) => {
                self.line_storage.put_chunk(chunk);
                let sd = self.get_screen_data();
                self.screen.update(sd);
            }
        }
        None
    }

    fn move_highlight_backward(&mut self) {
        let num_items = match self.last_screen_data {
            Some(ref sd) => sd.items.len(),
            None         => self.line_storage.lines.len(),
        };
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
        let num_items = match self.last_screen_data {
            Some(ref sd) => sd.items.len(),
            None         => self.line_storage.lines.len(),
        };
        if num_items == 0 {
            self.highlighted_row = 0;
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
        let num_items = match self.last_screen_data {
            Some(ref sd) => sd.items.len(),
            None         => self.line_storage.lines.len(),
        };
        if self.item_index + self.screen.list_view_height() < num_items {
            self.item_index += 1;
        }
    }

    fn get_screen_data(&mut self) -> ScreenData {
        let sd = ScreenData {
            cursor_index: self.query_editor.cursor_position(),
            highlighted_row: self.highlighted_row,
            item_index: self.item_index,
            items: self.get_items(),
            query_string: self.get_query_string(),
            total_lines: self.line_storage.lines.len(),
        };
        self.last_screen_data = Some(sd.clone());
        sd
    }

    fn get_items(&self) -> Arc<Vec<Item>> {
        if let Some(ref last) = self.last_screen_data {
            if last.total_lines == self.line_storage.lines.len()
                && &*last.query_string == self.query_editor.as_str()
            {
                return last.items.clone();
            }
        }
        let query = self.query_editor.query();
        Arc::new(self.line_storage.find(&query))
    }

    fn get_query_string(&self) -> Arc<String> {
        if let Some(ref last) = self.last_screen_data {
            if &*last.query_string == self.query_editor.as_str() {
                return last.query_string.clone()
            }
        }
        Arc::new(self.query_editor.as_ref().to_string())
    }
}

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
                if query.test(line) {
                    Some(Item::new(line.clone()))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn put_chunk(&mut self, chunk: Vec<Arc<String>>) {
        self.lines.extend(chunk);
    }
}
