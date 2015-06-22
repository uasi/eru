use std::ops::Range;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender};

use item_list::ItemList;
use key::Key;
use line_index_cache::LineIndexCache;
use line_storage::LineStorage;
use query::{Query, QueryEditor};
use screen::Screen;
use screen_data::ScreenData;

pub struct State {
    item_list: ItemList,
    line_index_cache: LineIndexCache,
    line_storage: Arc<RwLock<LineStorage>>,
    query_editor: QueryEditor,
    screen: Screen,
}

pub enum StateInput {
    PutKey(Key),
    PutSearchResult(Query, Vec<usize>, Range<usize>),
    UpdateScreen,
}

pub enum StateReply {
    Complete(Option<Vec<Arc<String>>>),
    RequestSearch(Query, usize),
}

impl State {
    pub fn new(line_storage: Arc<RwLock<LineStorage>>, screen: Screen) -> Self {
        State {
            item_list: ItemList::new(screen.list_view_height()),
            line_index_cache: LineIndexCache::new(),
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
                let indices = self.item_list.selected_line_indices();
                let items = self.line_storage.read().unwrap().get_many_unchecked(indices);
                return Some(Complete(Some(items)));
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
                    let query_string = self.query_editor.as_ref().to_string();
                    if let Some(&(ref indices, end)) = self.line_index_cache.get(&query_string) {
                        self.item_list.set_line_indices(indices.clone());
                        self.screen.update(self.get_screen_data());
                        if end != self.line_storage.read().unwrap().len() {
                            return Some(RequestSearch(self.query_editor.query(), end));
                        }
                    } else {
                        return Some(RequestSearch(self.query_editor.query(), 0));
                    }
                } else {
                    self.item_list.set_line_index_range(0..self.line_storage.read().unwrap().len());
                    self.screen.update(self.get_screen_data());
                }
            }
            PutSearchResult(query, line_indices, range) => {
                let query_string = query.as_ref().to_string();
                self.line_index_cache.put(query_string.clone(), line_indices, range);
                let &(ref line_indices, end) = self.line_index_cache.get(&query_string).unwrap();
                self.item_list.set_line_indices(line_indices.clone());
                self.screen.update(self.get_screen_data());
                if &query_string == self.query_editor.as_ref() && end < self.line_storage.read().unwrap().len() {
                    return Some(RequestSearch(query, end));
                }
            }
            UpdateScreen => {
                if self.query_editor.as_ref().len() > 0 {
                    let query_string = self.query_editor.as_ref().to_string();
                    if let Some(&(ref indices, end)) = self.line_index_cache.get(&query_string) {
                        self.item_list.set_line_indices(indices.clone());
                        self.screen.update(self.get_screen_data());
                        if end != self.line_storage.read().unwrap().len() {
                            return Some(RequestSearch(self.query_editor.query(), end));
                        }
                    } else {
                        return Some(RequestSearch(self.query_editor.query(), 0));
                    }
                } else {
                    self.item_list.set_line_index_range(0..self.line_storage.read().unwrap().len());
                    self.screen.update(self.get_screen_data());
                }
            }
        }
        None
    }

    fn get_screen_data(&self) -> ScreenData {
        let indices = self.item_list.line_indices_in_clipping_range();
        let items = self.line_storage.read().unwrap().get_many_unchecked(indices);
        ScreenData {
            cursor_index: self.query_editor.cursor_position(),
            highlighted_row: self.item_list.highlighted_row(),
            item_list_len: self.item_list.len(),
            items: items,
            query_string: Arc::new(self.query_editor.as_ref().to_string()),
            total_lines: self.line_storage.read().unwrap().len(),
        }
    }
}
