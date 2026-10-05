//! QEMU's `isa-debug-exit` device: writing a value `v` to port 0xF4 makes
//! QEMU exit with status `(v << 1) | 1`. Only tests rely on this; on real
//! hardware the write hits nothing.
use crate::port::outl;

const PORT: u16 = 0xF4;

#[repr(u32)]
pub enum ExitCode {
    /// QEMU exit status 35.
    Failure = 0x11,
}

pub fn exit(code: ExitCode) -> ! {
    outl(PORT, code as u32);
    // Not under QEMU: nothing happened, so park.
    crate::halt()
}
