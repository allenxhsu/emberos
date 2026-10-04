#!/usr/bin/env bash
# Error path: a kernel built with --features selftest-panic must report the
# panic on serial and exit QEMU through isa-debug-exit with code 0x11,
# which QEMU surfaces as exit status (0x11 << 1) | 1 = 35.
set -euo pipefail
source "$(dirname "$0")/lib.sh"
ISO=${1:-$ROOT/build/emberos-panic.iso}

boot_iso "$ISO" panic
code=0; wait_for_exit || code=$?
[ "$code" -eq 35 ] || fail "expected QEMU exit 35 (panic), got $code"
grep -q '^panic: ' "$SERIAL" || fail "no 'panic: ' line on serial"
grep -q 'selftest' "$SERIAL" || fail "panic message missing"
! grep -q '^ready$' "$SERIAL" || fail "kernel printed 'ready' after a panic"
pass "panic: reported on serial and exited with 35"
