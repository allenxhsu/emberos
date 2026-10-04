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
| 2. Traps | GDT, IDT, exception handlers, a panic that prints a register dump | planned |
| 3. Ticks | PIC/APIC, timer and keyboard interrupts, echo keystrokes | planned |
| 4. Memory | Physical frame allocator from the Limine memory map, new page tables | planned |
| 5. Heap | Kernel heap, `alloc` crate, `Vec` and `Box` in the kernel | planned |
| 6. Tasks | Cooperative then preemptive kernel threads | planned |
| 7. Userland | Ring 3, syscalls, a first user program | planned |

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

## Build and run

Prerequisites: Rust via `rustup`, `qemu`, `xorriso`, a C compiler for the
Limine host tool. On macOS: `brew install rustup qemu xorriso`.

```
make toolchain   # once
make run         # build ISO, boot in QEMU, serial on your terminal
make test        # headless boot + panic tests
```

Serial output of milestone 1:

```
EmberOS v0.1.0
framebuffer: 1280x800 32bpp
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
