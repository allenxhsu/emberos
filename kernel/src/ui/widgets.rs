//! 3D building blocks: bevelled edges, buttons, window frames.
use super::icons::{self, Icon};
use super::theme::*;
use crate::font::{self, Weight};
use crate::framebuffer::{Framebuffer, Rect};

pub const TITLE_HEIGHT: usize = 18;
const TITLE_BUTTON: (usize, usize) = (16, 14);

/// One-pixel edge: `tl` along the top and left, `br` along the bottom and
/// right. The bottom and right lines own the corners.
fn edge(fb: &mut Framebuffer, r: Rect, tl: u32, br: u32) {
    fb.fill_rect(Rect::new(r.x, r.y, r.w.saturating_sub(1), 1), tl);
    fb.fill_rect(Rect::new(r.x, r.y, 1, r.h.saturating_sub(1)), tl);
    fb.fill_rect(Rect::new(r.x, r.y + r.h.saturating_sub(1), r.w, 1), br);
    fb.fill_rect(Rect::new(r.x + r.w.saturating_sub(1), r.y, 1, r.h), br);
}

/// A raised grey slab, the body of every button.
pub fn raised(fb: &mut Framebuffer, r: Rect) {
    fb.fill_rect(r, FACE);
    edge(fb, r, HIGHLIGHT, DARK);
    edge(fb, r.inset(1), LIGHT, SHADOW);
}

/// A one-pixel sunken well, as around the tray clock.
pub fn sunken(fb: &mut Framebuffer, r: Rect) {
    edge(fb, r, SHADOW, HIGHLIGHT);
}

/// `text` centred in `r`.
pub fn label(fb: &mut Framebuffer, r: Rect, text: &str, colour: u32, weight: Weight) {
    let x = r.x + r.w.saturating_sub(font::width(text, weight)) / 2;
    let y = r.y + r.h.saturating_sub(font::HEIGHT) / 2;
    font::draw(fb, x, y, text, colour, weight);
}

/// The default push button of a dialog: a black ring around a raised slab.
pub fn default_button(fb: &mut Framebuffer, r: Rect, text: &str) {
    fb.fill_rect(r, DARK);
    raised(fb, r.inset(1));
    label(fb, r, text, TEXT, Weight::Bold);
}

enum Glyph {
    Minimise,
    Maximise,
    Close,
}

fn title_button(fb: &mut Framebuffer, r: Rect, glyph: Glyph) {
    raised(fb, r);
    match glyph {
        Glyph::Minimise => fb.fill_rect(Rect::new(r.x + 4, r.y + 9, 6, 2), DARK),
        Glyph::Maximise => {
            let b = Rect::new(r.x + 3, r.y + 2, 9, 9);
            fb.fill_rect(b, DARK);
            fb.fill_rect(Rect::new(b.x + 1, b.y + 2, 7, 6), FACE);
        }
        Glyph::Close => {
            for i in 0..7 {
                fb.fill_rect(Rect::new(r.x + 4 + i, r.y + 3 + i, 2, 1), DARK);
                fb.fill_rect(Rect::new(r.x + 10 - i, r.y + 3 + i, 2, 1), DARK);
            }
        }
    }
}

/// A framed window with a title bar. Returns the client area.
pub fn window(fb: &mut Framebuffer, r: Rect, icon: &Icon, title: &str) -> Rect {
    fb.fill_rect(r, FACE);
    edge(fb, r, LIGHT, DARK);
    edge(fb, r.inset(1), HIGHLIGHT, SHADOW);

    let inner = r.inset(4);
    let bar = Rect::new(inner.x, inner.y, inner.w, TITLE_HEIGHT);
    fb.fill_rect(bar, TITLE);
    icons::draw(fb, icon, bar.x + 2, bar.y + 1, 1);
    font::draw(fb, bar.x + 22, bar.y + 1, title, TITLE_TEXT, Weight::Bold);

    let (bw, bh) = TITLE_BUTTON;
    let by = bar.y + 2;
    let close = (bar.x + bar.w).saturating_sub(bw + 2);
    let maximise = close.saturating_sub(bw + 2);
    let minimise = maximise.saturating_sub(bw);
    title_button(fb, Rect::new(close, by, bw, bh), Glyph::Close);
    title_button(fb, Rect::new(maximise, by, bw, bh), Glyph::Maximise);
    title_button(fb, Rect::new(minimise, by, bw, bh), Glyph::Minimise);

    Rect::new(
        inner.x,
        bar.y + TITLE_HEIGHT,
        inner.w,
        inner.h.saturating_sub(TITLE_HEIGHT),
    )
}
