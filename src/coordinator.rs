use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use commander;
use line::Line;
use reader;
use searcher;
use state;

const POLLING_INTERVAL_MS: u32 = 10;

enum LoopCond {
    Break,
    Continue,
}

pub struct Coordinator {
    commander_rx: Receiver<commander::Event>,
    reader_rx: Receiver<reader::Event>,
    searcher_input_tx: Sender<searcher::Input>,
    searcher_reply_rx: Receiver<searcher::Reply>,
    state_input_tx: Sender<state::Input>,
    state_reply_rx: Receiver<state::Reply>,
}

impl Coordinator {
    pub fn new(
        commander_rx: Receiver<commander::Event>,
        reader_rx: Receiver<reader::Event>,
        searcher_input_tx: Sender<searcher::Input>,
        searcher_reply_rx: Receiver<searcher::Reply>,
        state_input_tx: Sender<state::Input>,
        state_reply_rx: Receiver<state::Reply>,
    ) -> Self
    {
        Coordinator {
            commander_rx: commander_rx,
            reader_rx: reader_rx,
            searcher_input_tx: searcher_input_tx,
            searcher_reply_rx: searcher_reply_rx,
            state_input_tx: state_input_tx,
            state_reply_rx: state_reply_rx,
        }
    }

    pub fn start(self) -> Option<Vec<Arc<Line>>> {
        use std::sync::mpsc::TryRecvError::Empty;
        'EVENT_LOOP: loop {
            loop {
                match self.commander_rx.try_recv() {
                    Ok(event) => {
                        match self.process_commander_event(event) {
                            LoopCond::Continue => (),
                            LoopCond::Break    => break 'EVENT_LOOP,
                        }
                    }
                    Err(Empty) => break,
                    Err(_)     => panic!("commander terminated unexpectedly"),
                };
            }
            loop {
                match self.reader_rx.try_recv() {
                    Ok(event)  => self.process_reader_event(event),
                    Err(Empty) => break,
                    Err(_)     => panic!("reader terminated unexpectedly"),
                }
            }
            loop {
                match self.searcher_reply_rx.try_recv() {
                    Ok(reply)  => self.process_searcher_reply(reply),
                    Err(Empty) => break,
                    Err(_)     => panic!("searcher terminated unexpectedly"),
                }
            }
            loop {
                use state::Reply::*;
                match self.state_reply_rx.try_recv() {
                    Ok(Complete(lines)) => { return lines; }
                    Ok(reply)           => self.process_state_reply(reply),
                    Err(Empty)          => break,
                    Err(_)              => panic!("state terminated unexpectedly"),
                }
            }
            thread::sleep_ms(POLLING_INTERVAL_MS);
        }
        None
    }

    fn process_commander_event(&self, event: commander::Event) -> LoopCond {
        use commander::Event::*;
        use state::Input;
        use self::LoopCond::{Break, Continue};
        match event {
            KeyDown(key) => {
                self.state_input_tx.send(Input::PutKey(key)).is_ok() || return Break;
            }
        }
        Continue
    }

    fn process_reader_event(&self, event: reader::Event) {
        use reader::Event::*;
        use state::Input;
        match event {
            DidReadChunk => {
                let _dont_care = self.state_input_tx.send(Input::UpdateScreen).is_ok();
            }
        }
    }

    fn process_searcher_reply(&self, reply: searcher::Reply) {
        use searcher::Reply::*;
        use state::Input;
        match reply {
            DidSearch(response) => {
                let _dont_care = self.state_input_tx.send(Input::PutSearchResponse(response)).is_ok();
             }
        }
    }

    fn process_state_reply(&self, reply: state::Reply) {
        use state::Reply::*;
        use searcher::Input;
        match reply {
            Complete(_) => unreachable!(),
            SendSearchRequest(request) => {
                let _dont_care = self.searcher_input_tx.send(Input::Search(request)).is_ok();
            }
        }
    }
}
