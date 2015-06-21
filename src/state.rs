use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender};

use item_list::ItemList;
use key::Key;
use line_storage::LineStorage;
use query::{Query, QueryEditor};
use screen::Screen;
use screen_data::ScreenData;

pub struct State {
    item_list: ItemList,
    line_storage: Arc<RwLock<LineStorage>>,
    query_editor: QueryEditor,
    screen: Screen,
}

pub enum StateInput {
    PutKey(Key),
    PutSearchResult(Vec<usize>),
    UpdateScreen,
}

pub enum StateReply {
    Complete(Option<Vec<Arc<String>>>),
    RequestSearch(Query),
}

impl State {
    pub fn new(line_storage: Arc<RwLock<LineStorage>>, screen: Screen) -> Self {
        State {
            item_list: ItemList::new(screen.list_view_height()),
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
                return Some(Complete(Some(self.item_list.selected_items())));
            }
            PutKey(Key::CtrlN) => {
                self.item_list.move_highlight_forward();
                self.screen.update(self.get_screen_data());
            }
            PutKey(Key::CtrlP) => {
                self.item_list.move_highlight_backward();
                self.screen.update(self.get_screen_data());
            }
            PutKey(key) => {
                self.query_editor.put_key(key);
                if self.query_editor.as_ref().len() > 0 {
                    self.screen.update(self.get_screen_data());
                    return Some(RequestSearch(self.query_editor.query()));
                }
                self.item_list.set_items(self.line_storage.read().unwrap().get_all());
                self.screen.update(self.get_screen_data());
            }
            PutSearchResult(line_indices) => {
                let line_storage = self.line_storage.read().unwrap();
                let items = line_storage.get_many_unchecked(line_indices);
                self.item_list.set_items(items);
                self.screen.update(self.get_screen_data());
            }
            UpdateScreen => {
                if self.query_editor.as_ref().len() > 0 {
                    self.screen.update(self.get_screen_data());
                    return Some(RequestSearch(self.query_editor.query()));
                }
                self.item_list.set_items(self.line_storage.read().unwrap().get_all());
                self.screen.update(self.get_screen_data());
            }
        }
        None
    }

    fn get_screen_data(&self) -> ScreenData {
        ScreenData {
            cursor_index: self.query_editor.cursor_position(),
            highlighted_row: self.item_list.highlighted_row(),
            item_list_len: self.item_list.len(),
            items: self.item_list.items_in_clipping_range(),
            query_string: Arc::new(self.query_editor.as_ref().to_string()),
            total_lines: self.line_storage.read().unwrap().len(),
        }
    }
}
