use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::Sender;
use std::thread;

use config::Config;
use line::Line;
use line_storage::LineStorage;
use thread_util::spawn_with_name;

const DUMP_INTERVAL_MS: u32 = 20; // ~10,000 lines per dump on my laptop when piped to `find`

pub enum Event {
    DidReadChunk,
}

pub struct Reader {
    chunk: Arc<Mutex<Vec<Arc<Line>>>>,
    reader: thread::JoinHandle<()>,
    line_storage: Arc<RwLock<LineStorage>>,
}

impl Reader {
    pub fn new(config: Config, line_storage: Arc<RwLock<LineStorage>>) -> Self {
        let chunk = Arc::new(Mutex::new(Vec::new()));
        let chunk_ = chunk.clone();
        let reader = spawn_parked_reader(config, chunk_);
        Reader {
            chunk: chunk,
            reader: reader,
            line_storage: line_storage,
        }
    }

    pub fn start(self, tx: Sender<Event>) {
        use self::Event::*;
        self.reader.thread().unpark();
        loop {
            thread::sleep_ms(DUMP_INTERVAL_MS);
            let mut chunk = self.chunk.lock().unwrap();
            if chunk.len() > 0 {
                self.line_storage.write().unwrap().put_chunk(chunk.clone());
                chunk.clear();
                drop(chunk);
                tx.send(DidReadChunk).is_ok() || return;
            }
        }
    }
}

fn spawn_parked_reader(config: Config, chunk: Arc<Mutex<Vec<Arc<Line>>>>) -> thread::JoinHandle<()> {
    spawn_with_name("reader::reader", move || {
        thread::park();
        let mut buf_reader = BufReader::new(config.input_source());
        loop {
            let mut buf = Vec::new();
            let res = buf_reader.read_until(0xA, &mut buf);
            match res {
                Ok(_) if buf.len() > 0 => {
                    if buf.last() == Some(&0xA) {
                        buf.pop().unwrap();
                    }
                    let mut chunk = chunk.lock().unwrap();
                    chunk.push(Arc::new(Line(buf)));
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
