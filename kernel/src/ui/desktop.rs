//! Composes the one screen EmberOS shows: desktop, icons, a welcome window
//! and the taskbar. Geometry is specified in README.md ("Desktop") and
//! asserted by tests/check_desktop.py; change all three together.
use super::icons::{self, Icon};
use super::theme::*;
use super::widgets;
use crate::font::{self, Weight};
use crate::framebuffer::{Framebuffer, Rect};
use crate::rtc::Time;
use core::fmt::{self, Write};

const TASKBAR_HEIGHT: usize = 28;
const WINDOW: (usize, usize) = (420, 200);
const ICONS: [(&Icon, &str); 3] = [
    (&icons::COMPUTER, "Computer"),
    (&icons::FOLDER, "Documents"),
    (&icons::BIN, "Recycle Bin"),
];

pub fn draw(fb: &mut Framebuffer, time: Time) {
    let (w, h) = (fb.width(), fb.height());
    let work_height = h.saturating_sub(TASKBAR_HEIGHT);

    fb.fill_rect(Rect::new(0, 0, w, work_height), DESKTOP);
    for (i, (icon, name)) in ICONS.iter().enumerate() {
        desktop_icon(fb, icon, name, 16 + 76 * i);
    }
    welcome(fb, w, work_height);
    taskbar(fb, Rect::new(0, work_height, w, TASKBAR_HEIGHT), time);
}

fn desktop_icon(fb: &mut Framebuffer, icon: &Icon, name: &str, y: usize) {
    icons::draw(fb, icon, 38, y, 2);
    widgets::label(
        fb,
        Rect::new(0, y + 36, 108, font::HEIGHT),
        name,
        ICON_TEXT,
        Weight::Regular,
    );
}

fn welcome(fb: &mut Framebuffer, screen_width: usize, work_height: usize) {
    let (w, h) = WINDOW;
    let frame = Rect::new(
        screen_width.saturating_sub(w) / 2,
        work_height.saturating_sub(h) / 2,
        w,
        h,
    );
    let client = widgets::window(fb, frame, &icons::EMBER, "Welcome");

    let lines = [
        "Welcome to EmberOS!",
        concat!("Version ", env!("CARGO_PKG_VERSION")),
        "A learning operating system for x86_64,",
        "written from scratch in Rust.",
    ];
    for (i, line) in lines.iter().enumerate() {
        font::draw(
            fb,
            client.x + 12,
            client.y + 12 + 20 * i,
            line,
            TEXT,
            Weight::Regular,
        );
    }
    let ok = Rect::new(frame.x + (w - 75) / 2, frame.y + h - 39, 75, 23);
    widgets::default_button(fb, ok, "OK");
}

fn taskbar(fb: &mut Framebuffer, bar: Rect, time: Time) {
    fb.fill_rect(bar, FACE);
    fb.fill_rect(Rect::new(bar.x, bar.y + 1, bar.w, 1), HIGHLIGHT);
    let (y, height) = (bar.y + 4, 22);

    let start = Rect::new(2, y, 24 + font::width("Start", Weight::Bold) + 6, height);
    widgets::raised(fb, start);
    icons::draw(fb, &icons::EMBER, start.x + 4, start.y + 3, 1);
    font::draw(fb, start.x + 24, start.y + 3, "Start", TEXT, Weight::Bold);

    let clock_width = font::width("12:00 PM", Weight::Regular) + 16;
    let clock = Rect::new(
        bar.w.saturating_sub(clock_width + 2),
        y,
        clock_width,
        height,
    );
    widgets::sunken(fb, clock);
    let mut text = ClockText::default();
    // ClockText holds the longest possible time, so this cannot fail.
    let _ = write!(
        text,
        "{}:{:02} {}",
        (time.hour + 11) % 12 + 1,
        time.minute,
        if time.hour < 12 { "AM" } else { "PM" }
    );
    widgets::label(fb, clock, text.as_str(), TEXT, Weight::Regular);
}

/// Stack buffer for the tray clock; there is no heap yet.
#[derive(Default)]
struct ClockText {
    bytes: [u8; 8],
    len: usize,
}

impl ClockText {
    fn as_str(&self) -> &str {
        // Only `write_str` fills the buffer, and only with whole strs.
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }
}

impl Write for ClockText {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let end = self.len + s.len();
        self.bytes
            .get_mut(self.len..end)
            .ok_or(fmt::Error)?
            .copy_from_slice(s.as_bytes());
        self.len = end;
        Ok(())
    }
}
