use clap::{App, Arg, ArgMatches};
use libc::{c_int, isatty};
use std::io;
use std::fs::File;

#[derive(Clone)]
pub struct Config {
    pub input_file_path: Option<String>,
}

impl Config {
    pub fn with_args() -> Self {
        let m = get_arg_matches();
        Config {
            input_file_path: m.value_of("INPUT").map(|s| s.to_string()),
        }
    }

    pub fn input_source(&self) -> Box<io::Read> {
        if let Some(ref path) = self.input_file_path {
            if let Ok(f) = File::open(path) {
                return Box::new(f);
            }
        } else if !stdin_is_tty() {
            return Box::new(io::stdin());
        }
        Box::new(io::empty())
    }
}

fn get_arg_matches<'a>() -> ArgMatches<'a, 'a> {
    App::new("eru")
        .arg(Arg::with_name("INPUT")
             .index(1))
        .get_matches()
}

fn stdin_is_tty() -> bool {
    unsafe { isatty(0 as c_int) == (1 as c_int) }
}
