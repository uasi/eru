use std::fs::File;
use std::io::Read;
use std::sync::{Once};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use crate::libc_aux;
use crate::key::Key;
use crate::thread_util::spawn_with_name;

static INSTALL: Once = Once::new();
static CAUGHT_SIGWINCH: AtomicBool = AtomicBool::new(false);

pub enum Event {
    KeyDown(Key),
    SigWinch,
}

pub struct Commander;

impl Commander {
    pub fn new() -> Self {
        install_sigwinch_handler_once();
        Commander
    }

    pub fn start(self, tx: Sender<Event>) {
        use self::Event::*;
        let tx_ = tx.clone();
        spawn_with_name("commander::sigwinch_notifier", move || {
            loop {
                if CAUGHT_SIGWINCH.load(Ordering::Relaxed) {
                    CAUGHT_SIGWINCH.store(false, Ordering::Relaxed);
                    let _dont_care = tx_.send(SigWinch).is_ok();
                }
                thread::sleep(Duration::from_millis(50));
            }
        });
        let tty = File::open("/dev/tty").unwrap();
        for byte in tty.bytes() {
            let _dont_care = tx.send(KeyDown(Key::from_u32(byte.unwrap() as u32))).is_ok();
        }
    }
}

fn install_sigwinch_handler_once() {
    INSTALL.call_once(|| {
        unsafe {
            let h = sigwinch_handler as *mut libc::c_void as libc::sighandler_t;
            libc::signal(libc_aux::SIGWINCH, h);
        }
    });
}

extern fn sigwinch_handler(_sig: libc::c_int) {
    CAUGHT_SIGWINCH.store(true, Ordering::Relaxed);
}
