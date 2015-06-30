use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};

use commander;
use line::Line;
use reader;
use searcher;
use state;
use thread_util::spawn_with_name;

pub struct Coordinator;

impl Coordinator {
    pub fn new() -> Self {
        Coordinator
    }

    pub fn start(
        self,
        commander_rx: Receiver<commander::Event>,
        reader_rx: Receiver<reader::Event>,
        searcher_input_tx: Sender<searcher::Input>,
        searcher_reply_rx: Receiver<searcher::Reply>,
        state_input_tx: Sender<state::Input>,
        state_reply_rx: Receiver<state::Reply>,
    ) -> Option<Vec<Arc<Line>>>
    {
        let state_input_tx_ = state_input_tx.clone();
        spawn_with_name("coordinator::commander_event", move || {
            while let Ok(event) = commander_rx.recv() {
                process_commander_event(event, &state_input_tx_);
            }
        });

        let state_input_tx_ = state_input_tx.clone();
        spawn_with_name("coordinator::reader_event", move || {
            while let Ok(event) = reader_rx.recv() {
                process_reader_event(event, &state_input_tx_);
            }
        });

        spawn_with_name("coordinator::searcher_reply", move || {
            while let Ok(reply) = searcher_reply_rx.recv() {
                process_searcher_reply(reply, &state_input_tx);
            }
        });

        let handle = spawn_with_name("coordinator::state_reply", move || {
            while let Ok(reply) = state_reply_rx.recv() {
                match process_state_reply(reply, &searcher_input_tx) {
                    Some(lines) => return Some(lines),
                    _ => (),
                }
            }
            None
        });

        handle.join().unwrap_or(None)
    }
}

fn process_commander_event(event: commander::Event, tx: &Sender<state::Input>) {
    use commander::Event::*;
    use state::Input::*;
    match event {
        KeyDown(key) => {
            let _ = tx.send(PutKey(key)).is_ok();
        }
        SigWinch => {
            let _ = tx.send(ResizeScreen).is_ok();
        }
    }
}

fn process_reader_event(event: reader::Event, tx: &Sender<state::Input>) {
    use reader::Event::*;
    use state::Input::*;
    match event {
        DidFinish => {
            let _ = tx.send(ReaderDidFinish).is_ok();
        }
        DidReadChunk => {
            let _ = tx.send(UpdateScreen).is_ok();
        }
    }
}

fn process_searcher_reply(reply: searcher::Reply, tx: &Sender<state::Input>) {
    use searcher::Reply::*;
    use state::Input::*;
    match reply {
        DidSearch(response) => {
            let _ = tx.send(PutSearchResponse(response)).is_ok();
         }
    }
}

fn process_state_reply(reply: state::Reply, tx: &Sender<searcher::Input>) -> Option<Vec<Arc<Line>>> {
    use state::Reply::*;
    use searcher::Input::*;
    match reply {
        Complete(lines) => {
            Some(lines)
        }
        SendSearchRequest(request) => {
            let _ = tx.send(Search(request)).is_ok();
            None
        }
    }
}
