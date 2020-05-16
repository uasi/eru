use clap::{App, Arg, ArgMatches};
use libc::{c_int, isatty};
use std::io;
use std::fs::File;

#[derive(Clone)]
pub struct Config {
    initial_query: Option<String>,
    input_file_path: Option<String>,
    is_cjk: bool,
}

impl Config {
    pub fn with_args() -> Self {
        let m = get_arg_matches();
        Config {
            initial_query: m.value_of("query").map(|s| s.to_string()),
            input_file_path: m.value_of("INPUT").map(|s| s.to_string()),
            is_cjk: m.is_present("cjk"),
        }
    }

    pub fn initial_query(&self) -> Option<&str> {
        self.initial_query.as_ref().map(|s| s.as_ref())
    }

    pub fn input_source(&self) -> Box<dyn io::Read> {
        if let Some(ref path) = self.input_file_path {
            if let Ok(f) = File::open(path) {
                return Box::new(f);
            }
        } else if !stdin_is_tty() {
            return Box::new(io::stdin());
        }
        Box::new(io::empty())
    }

    pub fn is_cjk(&self) -> bool {
        self.is_cjk
    }
}

fn get_arg_matches<'a>() -> ArgMatches<'a> {
    App::new("eru")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("INPUT")
             .index(1))
        .arg(Arg::with_name("query")
             .long("query")
             .short("q")
             .takes_value(true))
        .arg(Arg::with_name("cjk")
             .long("cjk"))
        .get_matches()
}

fn stdin_is_tty() -> bool {
    unsafe { isatty(0 as c_int) == (1 as c_int) }
}
