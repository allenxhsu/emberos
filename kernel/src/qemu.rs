//! QEMU's `isa-debug-exit` device: writing a value `v` to port 0xF4 makes
//! QEMU exit with status `(v << 1) | 1`. Only tests rely on this; on real
//! hardware the write hits nothing.
use core::arch::asm;

const PORT: u16 = 0xF4;

#[repr(u32)]
pub enum ExitCode {
    /// QEMU exit status 35.
    Failure = 0x11,
}

pub fn exit(code: ExitCode) -> ! {
    // SAFETY: a 32-bit write to the debug-exit port touches no memory.
    unsafe {
        asm!("out dx, eax", in("dx") PORT, in("eax") code as u32, options(nomem, nostack, preserves_flags))
    }
    // Not under QEMU: nothing happened, so park.
    loop {
        // SAFETY: see `halt` in main.rs.
        unsafe { asm!("hlt", options(nomem, nostack, preserves_flags)) };
    }
}
