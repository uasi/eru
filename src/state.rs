use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, RwLock};

use crate::config::Config;
use crate::item_list::ItemList;
use crate::key::Key;
use crate::line::Line;
use crate::line_storage::LineStorage;
use crate::match_info_cache::MatchInfoCache;
use crate::query::QueryEditor;
use crate::screen::Screen;
use crate::screen_data::ScreenData;
use crate::search::{MatchInfo, Request, Response};

pub struct State {
    is_cjk: bool,
    item_list: ItemList,
    match_info_cache: MatchInfoCache,
    line_storage: Arc<RwLock<LineStorage>>,
    query_editor: QueryEditor,
    screen: Screen,
    status_message: Option<String>, // would be used to print debug messages
}

pub enum Input {
    PutKey(Key),
    PutSearchResponse(Response),
    ReaderDidFinish,
    ResizeScreen,
    UpdateScreen,
}

pub enum Reply {
    Complete(Vec<Arc<Line>>),
    SendSearchRequest(Request),
}

impl State {
    pub fn new(config: &Config, line_storage: Arc<RwLock<LineStorage>>, screen: Screen) -> Self {
        State {
            is_cjk: config.is_cjk(),
            item_list: ItemList::new(screen.list_view_height()),
            match_info_cache: MatchInfoCache::new(),
            line_storage,
            query_editor: QueryEditor::new(config.initial_query().unwrap_or(""), config.is_cjk()),
            screen,
            status_message: None,
        }
    }

    pub fn start(mut self, input_rx: Receiver<Input>, reply_tx: Sender<Reply>) {
        while let Ok(input) = input_rx.recv() {
            let reply = self.process_input(input);
            if let Some(reply) = reply {
                if reply_tx.send(reply).is_err() {
                    break;
                }
            }
        }
    }

    fn process_input(&mut self, input: Input) -> Option<Reply> {
        use self::Input::*;
        use self::Reply::*;
        match input {
            PutKey(Key::CtrlC) => {
                return Some(Complete(Vec::new()));
            }
            PutKey(Key::CtrlI) => {
                self.item_list.toggle_mark();
                self.item_list.move_highlight_forward();
                self.screen.update(self.get_screen_data());
            }
            PutKey(Key::CtrlM) => {
                let indices = self.item_list.selected_line_indices();
                let items = self
                    .line_storage
                    .read()
                    .unwrap()
                    .get_many_unchecked(indices);
                return Some(Complete(items));
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
                let query_str = self.query_editor.as_ref();
                if !query_str.is_empty() {
                    if let Some(&MatchInfo {
                        line_indices: ref indices,
                        ref index_range,
                    }) = self.match_info_cache.get(query_str)
                    {
                        let end = index_range.end;
                        self.item_list.set_line_indices(indices.clone());
                        self.screen.update(self.get_screen_data());
                        if end != self.line_storage.read().unwrap().len() {
                            let request = Request {
                                query: self.query_editor.query(),
                                start: end,
                            };
                            return Some(SendSearchRequest(request));
                        }
                    } else {
                        let request = Request {
                            query: self.query_editor.query(),
                            start: 0,
                        };
                        return Some(SendSearchRequest(request));
                    }
                } else {
                    self.item_list
                        .set_line_index_range(0..self.line_storage.read().unwrap().len());
                    self.screen.update(self.get_screen_data());
                }
            }
            PutSearchResponse(response) => {
                let Response { query, match_info } = response;
                let query_string = query.as_ref().to_owned();
                self.match_info_cache
                    .insert(query_string.clone(), match_info);
                let &MatchInfo {
                    ref line_indices,
                    ref index_range,
                } = self.match_info_cache.get(&query_string).unwrap();
                let end = index_range.end;
                self.item_list.set_line_indices(line_indices.clone());
                self.screen.update(self.get_screen_data());
                if query_string == self.query_editor.as_ref()
                    && end < self.line_storage.read().unwrap().len()
                {
                    let request = Request {
                        query: self.query_editor.query(),
                        start: end,
                    };
                    return Some(SendSearchRequest(request));
                }
            }
            ReaderDidFinish => {
                if self.line_storage.read().unwrap().len() == 0 {
                    return Some(Complete(Vec::new()));
                }
            }
            ResizeScreen => {
                self.screen.resize();
                self.screen.update(self.get_screen_data());
            }
            UpdateScreen => {
                let query_str = self.query_editor.as_ref();
                if !query_str.is_empty() {
                    if let Some(&MatchInfo {
                        line_indices: ref indices,
                        ref index_range,
                    }) = self.match_info_cache.get(query_str)
                    {
                        let end = index_range.end;
                        self.item_list.set_line_indices(indices.clone());
                        self.screen.update(self.get_screen_data());
                        if end != self.line_storage.read().unwrap().len() {
                            let request = Request {
                                query: self.query_editor.query(),
                                start: end,
                            };
                            return Some(SendSearchRequest(request));
                        }
                    } else {
                        let request = Request {
                            query: self.query_editor.query(),
                            start: 0,
                        };
                        return Some(SendSearchRequest(request));
                    }
                } else {
                    self.item_list
                        .set_line_index_range(0..self.line_storage.read().unwrap().len());
                    self.screen.update(self.get_screen_data());
                }
            }
        }
        None
    }

    fn get_screen_data(&self) -> ScreenData {
        let indices = self.item_list.line_indices_in_clipping_range();
        let items = self
            .line_storage
            .read()
            .unwrap()
            .get_many_unchecked(indices);
        ScreenData {
            cursor_index: self.query_editor.cursor_position(),
            highlighted_row: self.item_list.highlighted_row(),
            is_cjk: self.is_cjk,
            item_list_len: self.item_list.len(),
            items,
            marked_rows: self.item_list.marked_rows(),
            query_string: Arc::new(self.query_editor.as_ref().to_owned()),
            status_message: self.status_message.clone(),
            total_lines: self.line_storage.read().unwrap().len(),
        }
    }
}
