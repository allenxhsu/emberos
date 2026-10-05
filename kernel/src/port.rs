//! x86 port I/O. Every legacy device the kernel talks to goes through here.
use core::arch::asm;

#[inline]
pub fn outb(port: u16, value: u8) {
    // SAFETY: a port write touches no memory; callers pick ports they own.
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags))
    }
}

#[inline]
pub fn inb(port: u16) -> u8 {
    let value: u8;
    // SAFETY: a port read touches no memory.
    unsafe {
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags))
    }
    value
}

#[inline]
pub fn outl(port: u16, value: u32) {
    // SAFETY: a port write touches no memory; callers pick ports they own.
    unsafe {
        asm!("out dx, eax", in("dx") port, in("eax") value, options(nomem, nostack, preserves_flags))
    }
}
