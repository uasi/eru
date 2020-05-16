use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use config::Config;
use line::Line;
use line_storage::LineStorage;
use thread_util::spawn_with_name;

const DUMP_INTERVAL_MS: u64 = 20; // ~10,000 lines per dump on my laptop when piped to `find`

pub enum Event {
    DidFinish,
    DidReadChunk,
}

pub struct Reader {
    chunk: Arc<Mutex<Vec<Arc<Line>>>>,
    is_finished: Arc<AtomicBool>,
    line_storage: Arc<RwLock<LineStorage>>,
    reader: thread::JoinHandle<()>,
}

impl Reader {
    pub fn new(config: Config, line_storage: Arc<RwLock<LineStorage>>) -> Self {
        let chunk = Arc::new(Mutex::new(Vec::new()));
        let is_finished = Arc::new(AtomicBool::new(false));
        let reader = spawn_parked_reader(config, chunk.clone(), is_finished.clone());
        Reader {
            chunk: chunk,
            is_finished: is_finished,
            line_storage: line_storage,
            reader: reader,
        }
    }

    pub fn start(self, tx: Sender<Event>) {
        use self::Event::*;
        self.reader.thread().unpark();
        while !self.is_finished.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_millis(DUMP_INTERVAL_MS));
            let mut chunk = self.chunk.lock().unwrap();
            if chunk.len() > 0 {
                self.line_storage.write().unwrap().put_chunk(chunk.clone());
                chunk.clear();
                drop(chunk);
                if !tx.send(DidReadChunk).is_ok() {
                    return;
                }
            }
        }
        let _ = tx.send(DidFinish);
    }
}

fn spawn_parked_reader(config: Config, chunk: Arc<Mutex<Vec<Arc<Line>>>>, is_finished: Arc<AtomicBool>) -> thread::JoinHandle<()> {
    spawn_with_name("reader::reader", move || {
        thread::park();
        let mut buf = Vec::with_capacity(1024);
        let mut buf_reader = BufReader::new(config.input_source());
        loop {
            buf.clear();
            let res = buf_reader.read_until(0xA, &mut buf);
            match res {
                Ok(_) if buf.len() > 0 => {
                    if buf.last() == Some(&0xA) {
                        buf.pop().unwrap();
                    }
                    let mut chunk = chunk.lock().unwrap();
                    chunk.push(Arc::new(Line::new(buf.clone())));
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
        is_finished.store(true, Ordering::Relaxed);
    })
}
