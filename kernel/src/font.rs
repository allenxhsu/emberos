//! Monospace bitmap text. Glyphs are thresholded to on/off pixels: crisp,
//! un-antialiased text is part of the period look, and it means text can be
//! drawn over any background without blending.
use crate::framebuffer::Framebuffer;
use noto_sans_mono_bitmap::{FontWeight, RasterHeight, get_raster, get_raster_width};

pub const HEIGHT: usize = 16;
const SIZE: RasterHeight = RasterHeight::Size16;
const THRESHOLD: u8 = 112;

#[derive(Clone, Copy)]
pub enum Weight {
    Regular,
    Bold,
}

impl Weight {
    fn raw(self) -> FontWeight {
        match self {
            Weight::Regular => FontWeight::Regular,
            Weight::Bold => FontWeight::Bold,
        }
    }
}

/// Width in pixels of `text` drawn in `weight`.
pub fn width(text: &str, weight: Weight) -> usize {
    text.chars().count() * get_raster_width(weight.raw(), SIZE)
}

/// Draw `text` with its top-left corner at (x, y).
pub fn draw(fb: &mut Framebuffer, x: usize, y: usize, text: &str, colour: u32, weight: Weight) {
    let advance = get_raster_width(weight.raw(), SIZE);
    for (i, ch) in text.chars().enumerate() {
        let glyph = get_raster(ch, weight.raw(), SIZE)
            .or_else(|| get_raster('?', weight.raw(), SIZE))
            .expect("'?' is in the basic-latin font block");
        for (dy, row) in glyph.raster().iter().enumerate() {
            for (dx, &intensity) in row.iter().enumerate() {
                if intensity >= THRESHOLD {
                    fb.put_pixel(x + i * advance + dx, y + dy, colour);
                }
            }
        }
    }
}
