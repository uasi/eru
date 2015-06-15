use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use commander::CommanderEvent;
use reader::ReaderEvent;
use screen::ScreenEvent;
use state::{StateInput, StateReply};

const POLLING_INTERVAL_MS: u32 = 10;

enum LoopCond {
    Break,
    Continue,
}

pub struct Coordinator {
    commander_rx: Receiver<CommanderEvent>,
    reader_rx: Receiver<ReaderEvent>,
    screen_tx: Sender<ScreenEvent>,
    state_input_tx: Sender<StateInput>,
    state_reply_rx: Receiver<StateReply>,
}

impl Coordinator {
    pub fn new(
        commander_rx: Receiver<CommanderEvent>,
        reader_rx: Receiver<ReaderEvent>,
        screen_tx: Sender<ScreenEvent>,
        state_input_tx: Sender<StateInput>,
        state_reply_rx: Receiver<StateReply>,
    ) -> Self
    {
        Coordinator {
            commander_rx: commander_rx,
            reader_rx: reader_rx,
            screen_tx: screen_tx,
            state_input_tx: state_input_tx,
            state_reply_rx: state_reply_rx,
        }
    }

    pub fn start(self) -> Option<Vec<Arc<String>>> {
        use std::sync::mpsc::TryRecvError::Empty;
        let _dont_care = self.state_input_tx.send(StateInput::EmitUpdateScreen).is_ok();
        'EVENT_LOOP: loop {
            loop {
                match self.commander_rx.try_recv() {
                    Ok(ev) => {
                        match self.process_commander_event(ev) {
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
                    Ok(ev)     => self.process_reader_event(ev),
                    Err(Empty) => break,
                    Err(_)     => panic!("reader terminated unexpectedly"),
                }
            }
            let mut screen_data = None;
            loop {
                use state::StateReply::*;
                match self.state_reply_rx.try_recv() {
                    Ok(UpdateScreen(sd)) => { screen_data = Some(sd); }
                    Ok(Complete(lines))  => { return lines; }
                    Err(Empty)           => break,
                    Err(_)               => panic!("state terminated unexpectedly"),
                }
            }
            if let Some(sd) = screen_data {
                self.screen_tx.send(::screen::ScreenEvent::Update(sd)).is_ok() || break 'EVENT_LOOP;
            }
            thread::sleep_ms(POLLING_INTERVAL_MS);
        }
        Default::default()
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
            DidReadLine(line) => {
                let _dont_care = self.state_input_tx.send(StateInput::PutLine(line)).is_ok();
            }
            WillFinish => {
            }
            WillDie => {
            }
        }
    }
}
