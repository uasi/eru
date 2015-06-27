use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;

use commander::Commander;
use config::Config;
use coordinator::Coordinator;
use line_storage::LineStorage;
use reader::Reader;
use screen::{self, Screen};
use searcher::Searcher;
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
        screen::initialize();

        let (commander_tx, commander_rx) = channel();
        let (reader_tx, reader_rx) = channel();
        let (searcher_input_tx, searcher_input_rx) = channel();
        let (searcher_reply_tx, searcher_reply_rx) = channel();
        let (state_input_tx, state_input_rx) = channel();
        let (state_reply_tx, state_reply_rx) = channel();
        let line_storage = Arc::new(RwLock::new(LineStorage::new()));

        spawn_with_name("commander", move || {
            let commander = Commander::new();
            commander.start(commander_tx);
        });

        let config = self.config.clone();
        let line_storage_ = line_storage.clone();
        spawn_with_name("reader", move || {
            let reader = Reader::new(config, line_storage_);
            reader.start(reader_tx);
        });

        let line_storage_ = line_storage.clone();
        spawn_with_name("searcher", move || {
            let searcher = Searcher::new(line_storage_);
            searcher.start(searcher_input_rx, searcher_reply_tx);
        });

        let line_storage_ = line_storage.clone();
        spawn_with_name("state", move || {
            let state = State::new(line_storage_, Screen::new());
            state.start(state_input_rx, state_reply_tx);
        });

        let coordinator = Coordinator::new(
            commander_rx,
            reader_rx,
            searcher_input_tx,
            searcher_reply_rx,
            state_input_tx,
            state_reply_rx,
        );

        let result = coordinator.start();
        screen::finalize();
        result
    }
}
