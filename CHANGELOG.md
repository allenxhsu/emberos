# Changelog

## [Unreleased]

### Added
- Windows 95-style desktop drawn at boot: teal background, taskbar with Start button, desktop icons, and a welcome window.
- Tray clock showing the CMOS real-time clock.
- Boot on x86_64 via Limine, print a banner over COM1 serial, and draw it on the framebuffer.
- `make test`: headless QEMU boot test with a screenshot check, plus a panic-path test.
