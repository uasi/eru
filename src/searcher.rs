use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender};

use line_storage::LineStorage;
use query::Query;

pub enum SearcherInput {
    Search(Query),
}

pub enum SearcherReply {
    DidSearch(Vec<usize>),
}

pub struct Searcher {
    line_storage: Arc<RwLock<LineStorage>>,
}

impl Searcher {
    pub fn new(line_storage: Arc<RwLock<LineStorage>>) -> Self {
        Searcher {
            line_storage: line_storage,
        }
    }

    pub fn start(self, input_rx: Receiver<SearcherInput>, reply_tx: Sender<SearcherReply>) {
        loop {
            use self::SearcherInput::*;
            let reply = match input_rx.recv() {
                Ok(Search(query)) => self.search(query),
                Err(_) => return,
            };
            let _dont_care = reply_tx.send(reply).is_ok();
        }
    }

    fn search(&self, query: Query) -> SearcherReply {
        let mut line_indices = Vec::new();
        let line_storage = self.line_storage.read().unwrap();
        for (i, line) in line_storage.iter().enumerate() {
            if query.test(&line) {
                line_indices.push(i);
            }
        }
        SearcherReply::DidSearch(line_indices)
    }
}
