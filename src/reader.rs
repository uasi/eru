use std::io::{self, BufRead, BufReader};
use std::sync::mpsc::Sender;
use std::thread;

pub enum ReaderEvent {
    DidReadLine(String),
    WillDie,
    WillFinish,
}

pub struct Reader {
    source: Box<io::Read>,
}

impl Reader {
    pub fn new(source: Box<io::Read>) -> Self {
        Reader {
            source: source,
        }
    }

    pub fn start(self, tx: Sender<ReaderEvent>) {
        use self::ReaderEvent::*;
        let mut buf_reader = BufReader::new(self.source);
        loop {
            let mut buf = String::new();
            let res = buf_reader.read_line(&mut buf);
            match res {
                Ok(_) if buf.len() > 0 => {
                    buf.pop().unwrap(); // chomp newline
                    tx.send(DidReadLine(buf)).is_ok() || return;
                }
                Ok(_) => {
                    tx.send(WillFinish).is_ok() || return;
                    thread::park();
                }
                Err(_) => {
                    tx.send(WillDie).is_ok() || return;
                    break;
                }
            }
        }
    }
}
