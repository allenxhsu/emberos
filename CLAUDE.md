# EmberOS

@AGENTS.md — shared workflow rules (questions first, tests first, commit every change).

A learning kernel for x86_64, written in Rust, booted by Limine, run in QEMU.
README.md is the design doc and roadmap — read the relevant section.

## Commands

- `make toolchain` — one-time: add the `x86_64-unknown-none` Rust target
- `make iso` — build the kernel and `build/emberos.iso`
- `make run` — boot the ISO in QEMU with serial on the terminal
- `make test` — build both ISOs and run every suite in `tests/` headless
- `tests/boot.sh` — serial lines + screenshot checked by `tests/check_desktop.py`
- `tests/panic.sh` — error path: a panicking kernel exits QEMU with code 35

Run `make test` before every commit.

## Layout

- `kernel/src/main.rs` — entry (`kmain`), Limine requests, boot sequence
- `kernel/src/port.rs` — port I/O; the only place `in`/`out` instructions live
- `kernel/src/serial.rs` — COM1 driver and `print!`/`println!`
- `kernel/src/rtc.rs` — CMOS clock, polled
- `kernel/src/framebuffer.rs` — `Framebuffer`, `Rect`, clipped pixel and rect fills
- `kernel/src/font.rs` — bitmap text, thresholded (no anti-aliasing)
- `kernel/src/ui/` — Windows 95-style shell: `theme` colours, `widgets`
  bevels/buttons/windows, `icons` pixel art, `desktop` layout (the only public entry)
- `kernel/src/qemu.rs` — `isa-debug-exit` port, used by tests and the panic handler
- `kernel/linker.ld` — higher-half layout, Limine request sections
- `scripts/mkiso.sh` — wraps a kernel ELF and Limine into a bootable ISO
- `build/` — Limine checkout, ISOs, test logs (generated, never committed)

## Rules

- Serial is the test channel: every test asserts on lines the kernel prints
  to COM1. Keep the lines in `tests/*.sh` and `main.rs` in sync.
- The desktop layout lives in three places that must agree: README.md
  ("Desktop"), `kernel/src/ui/desktop.rs`, `tests/check_desktop.py`.
- Original artwork only: no Microsoft logos, fonts or icons.
- No `unsafe` without a comment saying which invariant makes it sound.
- Stable Rust only; no `build-std`, no nightly features.
- CHANGELOG.md: one line per item under `### Added` / `### Changed` /
  `### Fixed` in `[Unreleased]`.
