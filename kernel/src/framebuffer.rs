//! Linear framebuffer handed over by Limine, plus a monospace bitmap font.
use noto_sans_mono_bitmap::{FontWeight, RasterHeight, get_raster, get_raster_width};

pub const BLACK: u32 = 0x0000_0000;
pub const WHITE: u32 = 0x00FF_FFFF;

const FONT_HEIGHT: RasterHeight = RasterHeight::Size16;
const FONT_WEIGHT: FontWeight = FontWeight::Regular;

pub struct Framebuffer {
    base: *mut u8,
    width: usize,
    height: usize,
    pitch: usize,
    bytes_per_pixel: usize,
}

impl Framebuffer {
    /// # Safety
    /// `fb` must describe memory that is mapped, writable, and not used by
    /// anything else for the lifetime of the returned value.
    pub unsafe fn from_limine(fb: &limine::framebuffer::Framebuffer) -> Self {
        Self {
            base: fb.addr(),
            width: fb.width() as usize,
            height: fb.height() as usize,
            pitch: fb.pitch() as usize,
            bytes_per_pixel: (fb.bpp() as usize).div_ceil(8),
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, colour: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let offset = y * self.pitch + x * self.bytes_per_pixel;
        // SAFETY: bounds were checked against width/height, and `pitch`
        // rows of the framebuffer are mapped (from_limine's contract).
        unsafe {
            let px = self.base.add(offset) as *mut u32;
            px.write_volatile(colour);
        }
    }

    pub fn clear(&mut self, colour: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.put_pixel(x, y, colour);
            }
        }
    }

    /// Draw `text` with its top-left corner at (x, y). Returns the x after it.
    pub fn draw_text(&mut self, mut x: usize, y: usize, text: &str, colour: u32) -> usize {
        let advance = get_raster_width(FONT_WEIGHT, FONT_HEIGHT);
        for ch in text.chars() {
            let glyph = get_raster(ch, FONT_WEIGHT, FONT_HEIGHT)
                .or_else(|| get_raster('?', FONT_WEIGHT, FONT_HEIGHT))
                .expect("'?' is in the basic-latin font block");
            for (dy, row) in glyph.raster().iter().enumerate() {
                for (dx, &intensity) in row.iter().enumerate() {
                    if intensity > 0 {
                        self.put_pixel(x + dx, y + dy, scale(colour, intensity));
                    }
                }
            }
            x += advance;
        }
        x
    }
}

/// Multiply each 8-bit channel of an 0x00RRGGBB colour by `intensity / 255`.
fn scale(colour: u32, intensity: u8) -> u32 {
    let i = intensity as u32;
    let ch = |shift: u32| (((colour >> shift) & 0xFF) * i / 255) << shift;
    ch(16) | ch(8) | ch(0)
}
