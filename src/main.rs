extern crate libc;
extern crate ncurses;
extern crate unicode_width;

mod app;
mod commander;
mod config;
mod coordinator;
mod item;
mod item_list;
mod key;
mod libc_aux;
mod line;
mod line_index_cache;
mod line_storage;
mod pattern;
mod query;
mod reader;
mod screen;
mod screen_data;
mod search;
mod searcher;
mod state;
mod thread_util;
mod window;

use std::io::{self, Write};

use app::App;

fn main() {
    let app = App::new();
    if let Some(lines) = app.start() {
        let stdout_ = io::stdout();
        let mut stdout = stdout_.lock();
        for line in lines {
            stdout.write_all(line.as_ref()).unwrap();
            stdout.write_all("\n".as_ref()).unwrap();
        }
        stdout.flush();
    }
}
