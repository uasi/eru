use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;

use config::Config;
use thread_util::spawn_with_name;

const DUMP_INTERVAL_MS: u32 = 20; // ~10,000 lines per dump on my laptop when piped to `find`

pub enum ReaderEvent {
    DidReadChunk(Vec<String>),
}

pub struct Reader {
    chunk: Arc<Mutex<Vec<String>>>,
    reader: thread::JoinHandle<()>,
}

impl Reader {
    pub fn new(config: Config) -> Self {
        let chunk = Arc::new(Mutex::new(Vec::new()));
        let chunk_ = chunk.clone();
        let reader = spawn_parked_reader(config, chunk_);
        Reader {
            chunk: chunk,
            reader: reader,
        }
    }

    pub fn start(self, tx: Sender<ReaderEvent>) {
        use self::ReaderEvent::*;
        self.reader.thread().unpark();
        loop {
            thread::sleep_ms(DUMP_INTERVAL_MS);
            let mut chunk = self.chunk.lock().unwrap();
            if chunk.len() > 0 {
                tx.send(DidReadChunk(chunk.clone())).is_ok() || return;
                chunk.clear();
            }
        }
    }
}

fn spawn_parked_reader(config: Config, chunk: Arc<Mutex<Vec<String>>>) -> thread::JoinHandle<()> {
    spawn_with_name("reader::reader", move || {
        thread::park();
        let mut buf_reader = BufReader::new(config.input_source());
        loop {
            let mut buf = String::new();
            let res = buf_reader.read_line(&mut buf);
            match res {
                Ok(_) if buf.len() > 0 => {
                    buf.pop().unwrap(); // chomp newline
                    let mut chunk = chunk.lock().unwrap();
                    chunk.push(buf);
                    drop(chunk);
                }
                Ok(_) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }
    })
}
