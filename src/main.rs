extern crate libc;
extern crate ncurses;

mod app;
mod commander;
mod config;
mod coordinator;
mod item;
mod item_list;
mod key;
mod libc_aux;
mod line_storage;
mod pattern;
mod query;
mod reader;
mod screen;
mod screen_data;
mod searcher;
mod state;
mod thread_util;
mod window;

use app::App;

fn main() {
    screen::initialize();
    let app = App::new();
    if let Some(strings) = app.start() {
        screen::finalize();
        for string in strings {
            println!("{}", string);
        }
    } else {
        screen::finalize();
    }
}
