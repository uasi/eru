use std::sync::Arc;
use std::sync::mpsc::channel;

use commander::Commander;
use config::Config;
use coordinator::Coordinator;
use reader::Reader;
use screen::Screen;
use state::State;
use thread_util::spawn_with_name;

pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> Self {
        App {
            config: Config::with_args(),
        }
    }

    pub fn start(self) -> Option<Vec<Arc<String>>> {
        let (commander_tx, commander_rx) = channel();
        let (reader_tx, reader_rx) = channel();
        let (state_input_tx, state_input_rx) = channel();
        let (state_reply_tx, state_reply_rx) = channel();

        spawn_with_name("commander", move || {
            let commander = Commander::new();
            commander.start(commander_tx);
        });

        let config = self.config.clone();
        spawn_with_name("reader", move || {
            let reader = Reader::new(config);
            reader.start(reader_tx);
        });

        spawn_with_name("state", move || {
            let state = State::new(Screen::new());
            state.start(state_input_rx, state_reply_tx);
        });

        let coordinator = Coordinator::new(
            commander_rx,
            reader_rx,
            state_input_tx,
            state_reply_rx,
        );
        coordinator.start()
    }
}
