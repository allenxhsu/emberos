#!/usr/bin/env bash
# Milestone 1: the kernel boots, announces itself on COM1, finds the Limine
# framebuffer, draws on it, and settles into a halt loop without exiting.
set -euo pipefail
source "$(dirname "$0")/lib.sh"
ISO=${1:-$ROOT/build/emberos.iso}
SHOT="$OUT/boot.screen.ppm"; rm -f "$SHOT"

boot_iso "$ISO" boot
wait_for_line '^ready$' || fail "kernel never printed 'ready'"

grep -qE '^EmberOS v[0-9]+\.[0-9]+\.[0-9]+$' "$SERIAL" || fail "no version banner on serial"
grep -qE '^framebuffer: [0-9]+x[0-9]+ [0-9]+bpp$' "$SERIAL" || fail "no framebuffer line on serial"
! grep -q '^framebuffer: none$' "$SERIAL" || fail "kernel found no framebuffer"
! grep -qi 'panic' "$SERIAL" || fail "kernel panicked"

# The kernel must still be alive: it halts, it does not exit QEMU.
kill -0 "$QPID" 2>/dev/null || fail "QEMU exited; the kernel should halt, not exit"

monitor "screendump $SHOT"
[ -s "$SHOT" ] || fail "screendump produced no file"
read -r zeros nonzeros < <(ppm_stats "$SHOT")
[ "$nonzeros" -gt 500 ] || fail "screen is blank: only $nonzeros non-black bytes"
[ "$zeros" -gt 10000 ] || fail "screen has no black background: only $zeros zero bytes"

monitor quit
wait "$QPID" || true
pass "boot: banner, framebuffer ${nonzeros}B lit, halted"
