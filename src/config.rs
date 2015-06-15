use libc::{c_int, isatty};
use std::env;
use std::io;
use std::fs::File;

#[derive(Clone)]
pub struct Config {
    pub input_file_path: Option<String>,
}

impl Config {
    pub fn with_args() -> Self {
        let first_arg = env::args().nth(1);
        Config {
            input_file_path: first_arg,
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

fn stdin_is_tty() -> bool {
    unsafe { isatty(0 as c_int) == (1 as c_int) }
}
