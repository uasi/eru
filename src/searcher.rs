use std::cmp;
use std::ops::Range;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender};

use line_storage::LineStorage;
use query::Query;

pub enum SearcherInput {
    Search(Query, usize),
}

pub enum SearcherReply {
    DidSearch(Query, Vec<usize>, Range<usize>),
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
                Ok(Search(query, start)) => self.search(query, start),
                Err(_) => return,
            };
            let _dont_care = reply_tx.send(reply).is_ok();
        }
    }

    fn search(&self, query: Query, start: usize) -> SearcherReply {
        let tests_per_req = 5000;
        let mut line_indices = Vec::new();
        let line_storage = self.line_storage.read().unwrap();
        for (i, line) in line_storage.iter().enumerate().skip(start).take(tests_per_req) {
            if query.test(&line) {
                line_indices.push(i);
            }
        }
        let end = cmp::min(start + tests_per_req, line_storage.len());
        SearcherReply::DidSearch(query, line_indices, start..end)
    }
}
