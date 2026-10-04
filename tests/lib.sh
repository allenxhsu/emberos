# Shared helpers for the QEMU-based test suites. Source, don't run.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$ROOT/build/test"
mkdir -p "$OUT"

QEMU=(qemu-system-x86_64 -M q35 -m 128M -boot d -vga std -display none
      -device isa-debug-exit,iobase=0xf4,iosize=0x04 -no-reboot -no-shutdown)
TIMEOUT_TICKS=${TIMEOUT_TICKS:-300}   # 0.1 s each → 30 s

fail() { echo "FAIL: $*" >&2; [ -f "${SERIAL:-}" ] && { echo "--- serial ---" >&2; cat "$SERIAL" >&2; }; exit 1; }
pass() { echo "PASS: $*"; }

# boot_iso <iso> <name> — starts QEMU in the background; sets SERIAL, MON, QPID.
boot_iso() {
  local iso=$1 name=$2
  [ -f "$iso" ] || fail "$iso not built (run: make iso)"
  SERIAL="$OUT/$name.serial.log"; MON="$OUT/$name.monitor.sock"
  rm -f "$SERIAL" "$MON"
  "${QEMU[@]}" -cdrom "$iso" -serial "file:$SERIAL" -monitor "unix:$MON,server,nowait" &
  QPID=$!
}

# wait_for_line <regex> — polls the serial log until it matches or QEMU exits.
wait_for_line() {
  local i
  for ((i = 0; i < TIMEOUT_TICKS; i++)); do
    [ -f "$SERIAL" ] && grep -qE "$1" "$SERIAL" && return 0
    kill -0 "$QPID" 2>/dev/null || return 1
    sleep 0.1
  done
  return 1
}

# wait_for_exit — returns QEMU's exit code, or fails on timeout.
wait_for_exit() {
  local i
  for ((i = 0; i < TIMEOUT_TICKS; i++)); do
    kill -0 "$QPID" 2>/dev/null || { wait "$QPID"; return $?; }
    sleep 0.1
  done
  kill "$QPID" 2>/dev/null; fail "QEMU still running after timeout"
}

# monitor <cmd>... — sends commands to the QEMU monitor socket.
monitor() { { printf '%s\n' "$@"; sleep 1; } | nc -U "$MON" >/dev/null; }

# ppm_stats <file> — prints "<zero-bytes> <nonzero-bytes>" of the pixel data.
ppm_stats() {
  local total nz
  head -c 2 "$1" | grep -q '^P6$' || fail "$1 is not a binary PPM"
  total=$(wc -c < "$1" | tr -d ' ')
  nz=$(tr -d '\000' < "$1" | wc -c | tr -d ' ')
  echo "$((total - nz)) $nz"
}
