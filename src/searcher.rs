use std::cmp;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Receiver, Sender};

use line_storage::LineStorage;
use search::{Request, Response};

pub enum Input {
    Search(Request),
}

pub enum Reply {
    DidSearch(Response),
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

    pub fn start(self, input_rx: Receiver<Input>, reply_tx: Sender<Reply>) {
        loop {
            use self::Input::*;
            let reply = match input_rx.recv() {
                Ok(Search(request)) => self.search(request),
                Err(_) => return,
            };
            let _dont_care = reply_tx.send(reply).is_ok();
        }
    }

    fn search(&self, request: Request) -> Reply {
        let Request { query, start } = request;
        let tests_per_req = 5000;
        let mut line_indices = Vec::new();
        let line_storage = self.line_storage.read().unwrap();
        for (i, line) in line_storage.iter().enumerate().skip(start).take(tests_per_req) {
            if query.test(line.as_ref()) {
                line_indices.push(i);
            }
        }
        let end = cmp::min(start + tests_per_req, line_storage.len());
        let response = Response::new(query, line_indices, start..end);
        Reply::DidSearch(response)
    }
}
