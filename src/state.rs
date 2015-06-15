use std::cmp;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use item::Item;
use key::Key;
use query::{Query, QueryEditor};
use screen_data::ScreenData;

pub struct State {
    highlight_index: usize,
    last_screen_data: Option<ScreenData>,
    line_storage: LineStorage,
    query_editor: QueryEditor,
}

pub enum StateInput {
    EmitUpdateScreen,
    PutKey(Key),
    PutLine(String),
}

pub enum StateReply {
    Complete(Option<Vec<Arc<String>>>),
    UpdateScreen(ScreenData),
}

impl State {
    pub fn new() -> Self {
        State {
            highlight_index: 0,
            last_screen_data: None,
            line_storage: LineStorage::new(),
            query_editor: QueryEditor::new(),
        }
    }

    pub fn start(mut self, input_rx: Receiver<StateInput>, reply_tx: Sender<StateReply>) {
        loop {
            match input_rx.recv() {
                Ok(input) => {
                    let reply = self.process_input(input);
                    reply_tx.send(reply).is_ok() || break;
                }
                Err(_) => break,
            }
        }
    }

    fn process_input(&mut self, input: StateInput) -> StateReply {
        use key::Key;
        use self::StateInput::*;
        use self::StateReply::*;
        match input {
            EmitUpdateScreen => {
                UpdateScreen(self.get_screen_data())
            }
            PutKey(Key::CtrlM) => {
                let sd = self.get_screen_data();
                Complete(sd.items.get(self.highlight_index).and_then(|i| Some(vec![i.string.clone()])))
            }
            PutKey(Key::CtrlN) => {
                let num_items = match self.last_screen_data {
                    Some(ref sd) => sd.items.len(),
                    None         => self.line_storage.lines.len(),
                };
                self.highlight_index = match num_items {
                    0 => 0,
                    _ => cmp::min(self.highlight_index + 1, num_items - 1),
                };
                UpdateScreen(self.get_screen_data())
            }
            PutKey(Key::CtrlP) => {
                let num_items = match self.last_screen_data {
                    Some(ref sd) => sd.items.len(),
                    None         => self.line_storage.lines.len(),
                };
                self.highlight_index = match num_items {
                    0 => 0,
                    _ => cmp::min(cmp::max(self.highlight_index, 1) - 1, num_items - 1),
                };
                UpdateScreen(self.get_screen_data())
            }
            PutKey(key) => {
                self.query_editor.put_key(key);
                let num_items = self.get_screen_data().items.len();
                self.highlight_index = cmp::min(self.highlight_index, cmp::max(num_items, 1) - 1);
                UpdateScreen(self.get_screen_data())
            }
            PutLine(line) => {
                self.line_storage.put_line(line);
                UpdateScreen(self.get_screen_data())
            }
        }
    }

    fn get_screen_data(&mut self) -> ScreenData {
        let sd = ScreenData {
            cursor_index: self.query_editor.cursor_position(),
            highlight_index: self.highlight_index,
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

    pub fn put_line(&mut self, line: String) {
        self.lines.push(Arc::new(line));
    }
}
