//! EmberOS kernel entry. Limine drops us here in 64-bit long mode with paging
//! on, interrupts off, and the request structs below filled in.
#![no_std]
#![no_main]

mod framebuffer;
mod qemu;
mod serial;

use core::panic::PanicInfo;
use limine::BaseRevision;
use limine::request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker};

/// Limine protocol base revision we speak. Must be the first request.
#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[used]
#[unsafe(link_section = ".requests")]
static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

#[used]
#[unsafe(link_section = ".requests_start_marker")]
static _START_MARKER: RequestsStartMarker = RequestsStartMarker::new();
#[used]
#[unsafe(link_section = ".requests_end_marker")]
static _END_MARKER: RequestsEndMarker = RequestsEndMarker::new();

const BANNER: &str = concat!("EmberOS v", env!("CARGO_PKG_VERSION"));

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    serial::init();
    println!("{BANNER}");
    assert!(
        BASE_REVISION.is_supported(),
        "bootloader does not support Limine base revision 3"
    );

    if cfg!(feature = "selftest-panic") {
        panic!("selftest");
    }

    match FRAMEBUFFER_REQUEST
        .get_response()
        .and_then(|r| r.framebuffers().next())
    {
        Some(fb) => {
            println!(
                "framebuffer: {}x{} {}bpp",
                fb.width(),
                fb.height(),
                fb.bpp()
            );
            // SAFETY: Limine guarantees this framebuffer is mapped and ours
            // to draw on for as long as we use its page tables.
            let mut screen = unsafe { framebuffer::Framebuffer::from_limine(&fb) };
            screen.clear(framebuffer::BLACK);
            screen.draw_text(16, 16, BANNER, framebuffer::WHITE);
        }
        None => println!("framebuffer: none"),
    }

    println!("ready");
    halt()
}

/// Park the CPU. Interrupts are off, so `hlt` sleeps forever.
fn halt() -> ! {
    loop {
        // SAFETY: `hlt` has no memory effects; with IF clear it never returns.
        unsafe { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)) };
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("panic: {info}");
    qemu::exit(qemu::ExitCode::Failure)
}
