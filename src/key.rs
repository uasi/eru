use std::char;

#[derive(Copy, Clone)]
pub enum Key {
    CtrlA,
    CtrlB,
    CtrlC,
    CtrlD,
    CtrlE,
    CtrlF,
    CtrlG,
    CtrlH,
    CtrlI,
    CtrlJ,
    CtrlK,
    CtrlL,
    CtrlM,
    CtrlN,
    CtrlO,
    CtrlP,
    CtrlQ,
    CtrlR,
    CtrlS,
    CtrlT,
    CtrlU,
    CtrlV,
    CtrlW,
    CtrlX,
    CtrlY,
    CtrlZ,
    Esc,
    Del,
    Char(char),
}

impl Key {
    pub fn from_u32(u: u32) -> Key {
        use ::key::Key::*;
        let ch = char::from_u32(u).unwrap();
        match ch {
            '\x01' => CtrlA,
            '\x02' => CtrlB,
            '\x03' => CtrlC,
            '\x04' => CtrlD,
            '\x05' => CtrlE,
            '\x06' => CtrlF,
            '\x07' => CtrlG,
            '\x08' => CtrlH,
            '\x09' => CtrlI,
            '\x0A' => CtrlJ,
            '\x0B' => CtrlK,
            '\x0C' => CtrlL,
            '\x0D' => CtrlM,
            '\x0E' => CtrlN,
            '\x0F' => CtrlO,
            '\x10' => CtrlP,
            '\x11' => CtrlQ,
            '\x12' => CtrlR,
            '\x13' => CtrlS,
            '\x14' => CtrlT,
            '\x15' => CtrlU,
            '\x16' => CtrlV,
            '\x17' => CtrlW,
            '\x18' => CtrlX,
            '\x19' => CtrlY,
            '\x1A' => CtrlZ,
            '\x1B' => Esc,
            '\x7F' => Del,
            _      => Char(ch),
        }
    }
}
