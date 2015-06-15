use std::fs::File;
use std::io::Read;
use std::sync::mpsc::Sender;

use key::Key;

pub enum CommanderEvent {
    KeyDown(Key),
}

pub struct Commander;

impl Commander {
    pub fn new() -> Self {
        Commander
    }

    pub fn start(self, tx: Sender<CommanderEvent>) {
        use self::CommanderEvent::*;
        let tty = File::open("/dev/tty").unwrap();
        for byte in tty.bytes() {
            let _dont_care = tx.send(KeyDown(Key::from_u32(byte.unwrap() as u32))).is_ok();
        }
    }
}
