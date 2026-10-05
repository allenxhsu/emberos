#!/usr/bin/env bash
# The kernel boots, announces itself on COM1, reads the clock, draws the
# desktop on the Limine framebuffer, and halts without exiting. The desktop
# itself is checked pixel by pixel by check_desktop.py.
set -euo pipefail
source "$(dirname "$0")/lib.sh"
ISO=${1:-$ROOT/build/emberos.iso}
SHOT="$OUT/boot.screen.ppm"; rm -f "$SHOT"

boot_iso "$ISO" boot 4544
wait_for_line '^ready$' || fail "kernel never printed 'ready'"

grep -qE '^EmberOS v[0-9]+\.[0-9]+\.[0-9]+$' <(serial) || fail "no version banner on serial"
grep -qE '^framebuffer: [0-9]+x[0-9]+ [0-9]+bpp$' <(serial) || fail "no framebuffer line on serial"
! grep -q '^framebuffer: none$' <(serial) || fail "kernel found no framebuffer"
# QEMU's clock is pinned by lib.sh, so the time is exact.
grep -q '^clock: 15:04$' <(serial) || fail "no 'clock: 15:04' line on serial"
grep -q '^desktop: drawn$' <(serial) || fail "no 'desktop: drawn' line on serial"
! grep -qi 'panic' <(serial) || fail "kernel panicked"

# The kernel must still be alive: it halts, it does not exit QEMU.
kill -0 "$QPID" 2>/dev/null || fail "QEMU exited; the kernel should halt, not exit"

monitor "screendump $SHOT"
[ -s "$SHOT" ] || fail "screendump produced no file"
python3 "$ROOT/tests/check_desktop.py" "$SHOT" || fail "desktop does not match the UI concept"

monitor quit
wait "$QPID" || true
pass "boot: banner, clock, desktop drawn, halted"
