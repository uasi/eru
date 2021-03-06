extern crate clap;
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
mod line_storage;
mod match_info_cache;
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

fn main() {
    app::App::new().start();
}
