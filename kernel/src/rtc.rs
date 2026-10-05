//! CMOS real-time clock (MC146818), read by polling ports 0x70/0x71.
use crate::port::{inb, outb};

const INDEX: u16 = 0x70;
const DATA: u16 = 0x71;
const REG_MINUTE: u8 = 0x02;
const REG_HOUR: u8 = 0x04;
const REG_STATUS_A: u8 = 0x0A;
const REG_STATUS_B: u8 = 0x0B;
const UPDATE_IN_PROGRESS: u8 = 0x80;
const BINARY_MODE: u8 = 0x04;

/// Wall-clock time of day, 24-hour.
#[derive(Clone, Copy, PartialEq)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

fn read(reg: u8) -> u8 {
    outb(INDEX, reg);
    inb(DATA)
}

fn read_raw() -> Time {
    while read(REG_STATUS_A) & UPDATE_IN_PROGRESS != 0 {}
    Time {
        hour: read(REG_HOUR),
        minute: read(REG_MINUTE),
    }
}

pub fn now() -> Time {
    // Read until two samples agree, so we never see a half-updated clock.
    let mut raw = read_raw();
    loop {
        let again = read_raw();
        if again == raw {
            break;
        }
        raw = again;
    }
    if read(REG_STATUS_B) & BINARY_MODE != 0 {
        return raw;
    }
    let bcd = |v: u8| (v >> 4) * 10 + (v & 0x0F);
    Time {
        hour: bcd(raw.hour),
        minute: bcd(raw.minute),
    }
}
