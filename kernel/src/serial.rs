//! COM1 (16550 UART at I/O port 0x3F8). Polling, no interrupts, no locking:
//! milestone 1 runs one core with interrupts off, so there is no contention.
use core::arch::asm;
use core::fmt::{self, Write};

const COM1: u16 = 0x3F8;
const DATA: u16 = COM1;
const INT_ENABLE: u16 = COM1 + 1;
const FIFO_CTRL: u16 = COM1 + 2;
const LINE_CTRL: u16 = COM1 + 3;
const MODEM_CTRL: u16 = COM1 + 4;
const LINE_STATUS: u16 = COM1 + 5;

#[inline]
fn outb(port: u16, value: u8) {
    // SAFETY: writing a byte to a legacy UART port only affects that device.
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags))
    }
}

#[inline]
fn inb(port: u16) -> u8 {
    let value: u8;
    // SAFETY: reading a UART status register has no side effects on memory.
    unsafe {
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags))
    }
    value
}

/// 115200 baud, 8 data bits, no parity, one stop bit, FIFO on.
pub fn init() {
    outb(INT_ENABLE, 0x00);
    outb(LINE_CTRL, 0x80); // DLAB on: next two writes set the baud divisor
    outb(DATA, 0x01); // divisor low  = 1 -> 115200
    outb(INT_ENABLE, 0x00); // divisor high
    outb(LINE_CTRL, 0x03); // 8N1, DLAB off
    outb(FIFO_CTRL, 0xC7); // enable + clear FIFOs, 14-byte threshold
    outb(MODEM_CTRL, 0x0B); // DTR, RTS, OUT2
}

fn write_byte(byte: u8) {
    while inb(LINE_STATUS) & 0x20 == 0 {} // wait for transmit holding register empty
    outb(DATA, byte);
}

pub struct Serial;

impl Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            if b == b'\n' {
                write_byte(b'\r');
            }
            write_byte(b);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    // Serial::write_str never fails.
    let _ = Serial.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => { $crate::serial::print(format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => { $crate::serial::print(format_args!("{}\n", format_args!($($arg)*))) };
}
