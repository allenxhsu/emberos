//! Original 16x16 pixel art. Each row is 16 palette characters; see `colour`.
use crate::framebuffer::{Framebuffer, Rect};

pub type Icon = [&'static str; 16];

/// `.` is transparent.
fn colour(c: u8) -> Option<u32> {
    Some(match c {
        b'K' => 0x000000,
        b'W' => 0xFFFFFF,
        b'G' => 0xC0C0C0,
        b'D' => 0x808080,
        b'Y' => 0xFFFF00,
        b'y' => 0x808000,
        b'C' => 0x00FFFF,
        b'R' => 0xFF0000,
        b'O' => 0xFF8000,
        _ => return None,
    })
}

/// Draw `icon` at (x, y), each art pixel `scale` screen pixels square.
pub fn draw(fb: &mut Framebuffer, icon: &Icon, x: usize, y: usize, scale: usize) {
    for (row, line) in icon.iter().enumerate() {
        for (col, c) in line.bytes().enumerate() {
            if let Some(rgb) = colour(c) {
                fb.fill_rect(
                    Rect::new(x + col * scale, y + row * scale, scale, scale),
                    rgb,
                );
            }
        }
    }
}

/// The EmberOS mark: a small flame.
pub const EMBER: Icon = [
    "................",
    ".......R........",
    "......RR........",
    "......RRR.......",
    ".....RROR.......",
    "....RROORR.R....",
    "....ROOOORRR....",
    "...RROOYOORR....",
    "...ROOYYYOORR...",
    "...ROYYWYYOOR...",
    "...ROYWWWYYOR...",
    "...RROYWWYORR...",
    "....RROYYORR....",
    ".....RRRRRR.....",
    "................",
    "................",
];

pub const COMPUTER: Icon = [
    "................",
    ".DDDDDDDDDDDDD..",
    ".DGGGGGGGGGGGGK.",
    ".DGKKKKKKKKKWGK.",
    ".DGKCCCCCCCCWGK.",
    ".DGKCCCCCCCCWGK.",
    ".DGKCCCCCCCCWGK.",
    ".DGKCCCCCCCCWGK.",
    ".DGKCCCCCCCCWGK.",
    ".DGKWWWWWWWWWGK.",
    ".DGGGGGGGGGGGGK.",
    "..KKKKKKKKKKKKK.",
    "....DGGGGGGK....",
    ".DDDDDDDDDDDDDD.",
    ".DGGGGGGGGGGGGK.",
    ".KKKKKKKKKKKKKK.",
];

pub const FOLDER: Icon = [
    "................",
    "................",
    "..KKKKK.........",
    ".KYYYYYK........",
    ".KYYYYYYKKKKKK..",
    ".KYWWWWWWWWWWYK.",
    ".KYWYYYYYYYYYyK.",
    ".KYWYYYYYYYYYyK.",
    ".KYWYYYYYYYYYyK.",
    ".KYWYYYYYYYYYyK.",
    ".KYWYYYYYYYYYyK.",
    ".KYWYYYYYYYYYyK.",
    ".KYyyyyyyyyyyyK.",
    "..KKKKKKKKKKKKK.",
    "................",
    "................",
];

pub const BIN: Icon = [
    "................",
    "....KKKKKKK.....",
    "..KKWWWWWWWKK...",
    ".KWWGGGGGGGWWK..",
    ".KGWWWWWWWWWGK..",
    "..KGGGGGGGGGK...",
    "..KWGWGWGWGDK...",
    "..KWGWGWGWGDK...",
    "..KWGWGWGWGDK...",
    "..KWGWGWGWGDK...",
    "..KWGWGWGWGDK...",
    "..KWGWGWGWGDK...",
    "...KWGWGWGDK....",
    "...KWGWGWGDK....",
    "....KKKKKKK.....",
    "................",
];
