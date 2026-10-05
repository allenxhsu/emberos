# EmberOS

A learning operating system for x86_64, written from scratch in Rust.

The point is to understand how a machine goes from power-on to a running
program, one layer at a time: bootloader hand-off, serial output, interrupts,
paging, allocation, scheduling, user mode. Each layer is a milestone with its
own tests, and nothing is added until the layer below it is tested.

## Status

| Milestone | What it does | State |
|---|---|---|
| 1. Hello | Boot via Limine, print a banner on COM1 serial and the framebuffer, halt | done |
| 2. Desktop | Windows 95-style desktop drawn at boot: taskbar, Start button, clock, icons, a window. No input yet | done |
| 3. Traps | GDT, IDT, exception handlers, a panic that prints a register dump | planned |
| 4. Ticks | PIC/APIC, timer, keyboard and PS/2 mouse interrupts; a moving cursor | planned |
| 5. Memory | Physical frame allocator from the Limine memory map, new page tables | planned |
| 6. Heap | Kernel heap, `alloc` crate, `Vec` and `Box` in the kernel | planned |
| 7. Tasks | Kernel threads; windows you can drag, a working Start menu | planned |
| 8. Userland | Ring 3, syscalls, a first user program | planned |

## Decisions

- **x86_64 only.** Best-documented target; QEMU and real PCs both work.
- **Rust, stable channel.** `x86_64-unknown-none` is a tier-2 target with a
  prebuilt `core`, so no nightly and no `build-std`. `#![no_std]`, `#![no_main]`.
- **Limine boot protocol.** Limine loads a 64-bit ELF straight into long mode
  with paging on, a memory map, and a framebuffer. The kernel is linked at
  `0xffffffff80000000` (higher half) and asks for what it needs through static
  request structs in a `.requests` section (see `kernel/linker.ld`).
- **Serial is the test channel.** The kernel prints machine-checkable lines on
  COM1. Tests boot the ISO headless in QEMU and assert on those lines, take a
  framebuffer screenshot through the QEMU monitor, and use the
  `isa-debug-exit` device to turn a kernel panic into a QEMU exit code.
- **One crate, flat modules** until a module grows a real interface worth
  isolating. Then it becomes its own crate in the workspace.

## Desktop

EmberOS borrows the look of Windows 95: teal desktop, grey 3D chrome, navy
title bars. All artwork and the Start icon are original; nothing here is
Microsoft's. Milestone 2 draws one fixed screen at boot. It takes no input
until interrupts exist (milestone 4).

Layout, for a screen of W x H pixels. `tests/check_desktop.py` asserts
exactly this, so change both together.

- **Desktop**: `#008080` everywhere above the taskbar.
- **Taskbar**: the bottom 28 rows, face `#C0C0C0`, white highlight on its
  second row.
- **Start button**: raised button at (2, H-24), 22 high, a 16x16 ember icon
  and a bold "Start" label. Not clickable yet.
- **Clock**: sunken box on the same rows, right edge at W-3, showing the
  CMOS real-time clock as `3:04 PM`. Read once at boot; it does not tick yet.
- **Icons**: 32x32 pixel art at (38, 16 + 76*i) with a white label centred
  underneath: Computer, Documents, Recycle Bin.
- **Welcome window**: 420x200, centred in the area above the taskbar. Navy
  title bar 18 high inset 4 from the frame, with icon, bold white title and
  minimise / maximise / close buttons (16x14). Body text in black, and a
  default 75x23 OK button centred 16 above the bottom edge.

3D edges follow the Windows 95 scheme. Raised: white then `#DFDFDF` on the
top and left, black then `#808080` on the bottom and right. Window frames
swap the two top-left colours. Sunken: `#808080` top-left, white
bottom-right.

States: there is one. No framebuffer means no desktop, and the kernel says
`framebuffer: none` on serial and carries on. Screens smaller than the
window clip it rather than fail.

## Build and run

Prerequisites: Rust via `rustup`, `qemu`, `xorriso`, a C compiler for the
Limine host tool. On macOS: `brew install rustup qemu xorriso`.

```
make toolchain   # once
make run         # build ISO, boot in QEMU, serial on your terminal
make test        # headless boot + panic tests
```

Serial output:

```
EmberOS v0.2.0
clock: 15:04
framebuffer: 1280x800 32bpp
desktop: drawn
ready
```

## Layout

```
kernel/src/main.rs        kmain, Limine requests, boot sequence, panic handler
kernel/src/port.rs        x86 port I/O (inb/outb/outl)
kernel/src/serial.rs      COM1 (0x3F8) driver, print!/println!
kernel/src/rtc.rs         CMOS real-time clock
kernel/src/framebuffer.rs pixels and filled rectangles on the Limine framebuffer
kernel/src/font.rs        bitmap text
kernel/src/ui/            the desktop: theme, widgets, icons, layout
kernel/src/qemu.rs        isa-debug-exit
kernel/linker.ld`).
- **Serial is the test channel.** The kernel prints machine-checkable lines on
  COM1. Tests boot the ISO headless in QEMU and assert on those lines, take a
  framebuffer screenshot through the QEMU monitor, and use the
  `isa-debug-exit` device to turn a kernel panic into a QEMU exit code.
- **One crate, flat modules** until a module grows a real interface worth
  isolating. Then it becomes its own crate in the workspace.

## Desktop

EmberOS borrows the look of Windows 95: teal desktop, grey 3D chrome, navy
title bars. All artwork and the Start icon are original; nothing here is
Microsoft's. Milestone 2 draws one fixed screen at boot. It takes no input
until interrupts exist (milestone 4).

Layout, for a screen of W x H pixels. `tests/check_desktop.py` asserts
exactly this, so change both together.

- **Desktop**: `#008080` everywhere above the taskbar.
- **Taskbar**: the bottom 28 rows, face `#C0C0C0`, white highlight on its
  second row.
- **Start button**: raised button at (2, H-24), 22 high, a 16x16 ember icon
  and a bold "Start" label. Not clickable yet.
- **Clock**: sunken box on the same rows, right edge at W-3, showing the
  CMOS real-time clock as `3:04 PM`. Read once at boot; it does not tick yet.
- **Icons**: 32x32 pixel art at (38, 16 + 76*i) with a white label centred
  underneath: Computer, Documents, Recycle Bin.
- **Welcome window**: 420x200, centred in the area above the taskbar. Navy
  title bar 18 high inset 4 from the frame, with icon, bold white title and
  minimise / maximise / close buttons (16x14). Body text in black, and a
  default 75x23 OK button centred 16 above the bottom edge.

3D edges follow the Windows 95 scheme. Raised: white then `#DFDFDF` on the
top and left, black then `#808080` on the bottom and right. Window frames
swap the two top-left colours. Sunken: `#808080` top-left, white
bottom-right.

States: there is one. No framebuffer means no desktop, and the kernel says
`framebuffer: none` on serial and carries on. Screens smaller than the
window clip it rather than fail.

## Build and run

Prerequisites: Rust via `rustup`, `qemu`, `xorriso`, a C compiler for the
Limine host tool. On macOS: `brew install rustup qemu xorriso`.

```
make toolchain   # once
make run         # build ISO, boot in QEMU, serial on your terminal
make test        # headless boot + panic tests
```

Serial output:

```
EmberOS v0.2.0
clock: 15:04
framebuffer: 1280x800 32bpp
desktop: drawn
ready
```

## Layout

```
kernel/src/main.rs        kmain, Limine requests, boot sequence, panic handler
kernel/src/serial.rs      COM1 (0x3F8) driver, print!/println!
kernel/src/framebuffer.rs pixels and bitmap text on the Limine framebuffer
kernel/src/qemu.rs        isa-debug-exit
kernel/linker.ld          section layout Limine expects
limine.conf               bootloader menu (timeout 0, one entry)
scripts/mkiso.sh          ELF + Limine -> hybrid BIOS/UEFI ISO
tests/                    QEMU-driven test suites, run by `make test`
build/                    generated: Limine checkout, ISOs, test logs
```

## Memory map (milestone 1)

Limine maps the kernel at its link address in the higher half and identity
maps the lower half plus a higher-half direct map (HHDM) of all physical
memory. Milestone 1 uses only what Limine set up; milestone 4 replaces it.
