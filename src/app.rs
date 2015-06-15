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
        let (screen_tx, screen_rx) = channel();
        let (state_input_tx, state_input_rx) = channel();
        let (state_reply_tx, state_reply_rx) = channel();

        spawn_with_name("commander", move || {
            let commander = Commander::new();
            commander.start(commander_tx);
        });

        let config = self.config.clone();
        spawn_with_name("reader", move || {
            let reader = Reader::new(config.input_source());
            reader.start(reader_tx);
        });

        spawn_with_name("screen", move || {
            let screen = Screen::new();
            screen.start(screen_rx);
        });

        spawn_with_name("state", move || {
            let state = State::new();
            state.start(state_input_rx, state_reply_tx);
        });

        let coordinator = Coordinator::new(
            commander_rx,
            reader_rx,
            screen_tx,
            state_input_tx,
            state_reply_rx,
        );
        coordinator.start()
    }
}
