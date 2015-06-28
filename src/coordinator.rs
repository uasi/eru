use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use commander::CommanderEvent;
use line::Line;
use reader::ReaderEvent;
use searcher::{SearcherInput, SearcherReply};
use state::{StateInput, StateReply};

const POLLING_INTERVAL_MS: u32 = 10;

enum LoopCond {
    Break,
    Continue,
}

pub struct Coordinator {
    commander_rx: Receiver<CommanderEvent>,
    reader_rx: Receiver<ReaderEvent>,
    searcher_input_tx: Sender<SearcherInput>,
    searcher_reply_rx: Receiver<SearcherReply>,
    state_input_tx: Sender<StateInput>,
    state_reply_rx: Receiver<StateReply>,
}

impl Coordinator {
    pub fn new(
        commander_rx: Receiver<CommanderEvent>,
        reader_rx: Receiver<ReaderEvent>,
        searcher_input_tx: Sender<SearcherInput>,
        searcher_reply_rx: Receiver<SearcherReply>,
        state_input_tx: Sender<StateInput>,
        state_reply_rx: Receiver<StateReply>,
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
                use state::StateReply::*;
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

    fn process_commander_event(&self, event: CommanderEvent) -> LoopCond {
        use commander::CommanderEvent::*;
        use state::StateInput;
        use self::LoopCond::{Break, Continue};
        match event {
            KeyDown(key) => {
                self.state_input_tx.send(StateInput::PutKey(key)).is_ok() || return Break;
            }
        }
        Continue
    }

    fn process_reader_event(&self, event: ReaderEvent) {
        use reader::ReaderEvent::*;
        use state::StateInput;
        match event {
            DidReadChunk => {
                let _dont_care = self.state_input_tx.send(StateInput::UpdateScreen).is_ok();
            }
        }
    }

    fn process_searcher_reply(&self, reply: SearcherReply) {
        use searcher::SearcherReply::*;
        match reply {
            DidSearch(response) => {
                let _dont_care = self.state_input_tx.send(StateInput::PutSearchResponse(response)).is_ok();
             }
        }
    }

    fn process_state_reply(&self, reply: StateReply) {
        use state::StateReply::*;
        use searcher::SearcherInput;
        match reply {
            Complete(_) => unreachable!(),
            SendSearchRequest(request) => {
                let _dont_care = self.searcher_input_tx.send(SearcherInput::Search(request)).is_ok();
            }
        }
    }
}
