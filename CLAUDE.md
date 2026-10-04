# EmberOS

@AGENTS.md — shared workflow rules (questions first, tests first, commit every change).

A learning kernel for x86_64, written in Rust, booted by Limine, run in QEMU.
README.md is the design doc and roadmap — read the relevant section.

## Commands

- `make toolchain` — one-time: add the `x86_64-unknown-none` Rust target
- `make iso` — build the kernel and `build/emberos.iso`
- `make run` — boot the ISO in QEMU with serial on the terminal
- `make test` — build both ISOs and run every suite in `tests/` headless
- `tests/boot.sh` — milestone test: serial banner + framebuffer screenshot
- `tests/panic.sh` — error path: a panicking kernel exits QEMU with code 35

Run `make test` before every commit.

## Layout

- `kernel/src/main.rs` — entry (`kmain`), Limine requests, boot sequence
- `kernel/src/serial.rs` — COM1 driver and `print!`/`println!`
- `kernel/src/framebuffer.rs` — pixel + text drawing on the Limine framebuffer
- `kernel/src/qemu.rs` — `isa-debug-exit` port, used by tests and the panic handler
- `kernel/linker.ld` — higher-half layout, Limine request sections
- `scripts/mkiso.sh` — wraps a kernel ELF and Limine into a bootable ISO
- `build/` — Limine checkout, ISOs, test logs (generated, never committed)

## Rules

- Serial is the test channel: every test asserts on lines the kernel prints
  to COM1. Keep the lines in `tests/*.sh` and `main.rs` in sync.
- No `unsafe` without a comment saying which invariant makes it sound.
- Stable Rust only; no `build-std`, no nightly features.
- CHANGELOG.md: one line per item under `### Added` / `### Changed` /
  `### Fixed` in `[Unreleased]`.
