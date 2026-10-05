//! Linear framebuffer handed over by Limine: pixels and filled rectangles.
//! Colours are 0x00RRGGBB. Everything clips to the screen.

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
}

impl Rect {
    pub const fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Self { x, y, w, h }
    }

    /// The rectangle `n` pixels inside this one on every side.
    pub const fn inset(self, n: usize) -> Self {
        Self::new(
            self.x + n,
            self.y + n,
            self.w.saturating_sub(2 * n),
            self.h.saturating_sub(2 * n),
        )
    }
}

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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
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

    pub fn fill_rect(&mut self, r: Rect, colour: u32) {
        for y in r.y..(r.y + r.h).min(self.height) {
            for x in r.x..(r.x + r.w).min(self.width) {
                self.put_pixel(x, y, colour);
            }
        }
    }
}
