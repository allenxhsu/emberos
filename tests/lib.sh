# Shared helpers for the QEMU-based test suites. Source, don't run.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$ROOT/build/test"
mkdir -p "$OUT"

QEMU=(qemu-system-x86_64 -M q35 -m 128M -boot d -vga std -display none
      -device isa-debug-exit,iobase=0xf4,iosize=0x04 -no-reboot)
TIMEOUT_TICKS=${TIMEOUT_TICKS:-300}   # 0.1 s each → 30 s

# serial — the COM1 log with the kernel's \r\n line endings normalised.
serial() { [ -f "${SERIAL:-}" ] && tr -d '\r' < "$SERIAL" || true; }
fail() { echo "FAIL: $*" >&2; echo "--- serial ---" >&2; serial >&2; exit 1; }
pass() { echo "PASS: $*"; }

# boot_iso <iso> <name> <monitor-port> — starts QEMU in the background;
# sets SERIAL, MONPORT, QPID. The monitor listens on TCP so tests can drive
# it with bash's /dev/tcp and need no netcat (BSD and OpenBSD nc disagree
# about when to exit).
boot_iso() {
  local iso=$1 name=$2
  [ -f "$iso" ] || fail "$iso not built (run: make iso)"
  SERIAL="$OUT/$name.serial.log"; MONPORT=$3
  rm -f "$SERIAL"
  "${QEMU[@]}" -cdrom "$iso" -serial "file:$SERIAL" \
    -monitor "tcp:127.0.0.1:$MONPORT,server,nowait" \
    >"$OUT/$name.qemu.log" 2>&1 &
  QPID=$!
  # Never leave a QEMU behind: it would hold the caller's pipes open forever.
  trap 'kill "$QPID" 2>/dev/null || true' EXIT
}

# wait_for_line <regex> — polls the serial log until it matches or QEMU exits.
wait_for_line() {
  local i
  for ((i = 0; i < TIMEOUT_TICKS; i++)); do
    serial | grep -qE "$1" && return 0
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

# monitor <cmd>... — sends commands to the QEMU monitor over TCP.
monitor() {
  # Retry: a SIGCHLD from an exiting helper interrupts connect() and bash 3.2
  # does not restart it, and QEMU may still be starting up.
  local i
  for ((i = 0; i < 20; i++)); do
    { exec 3<>"/dev/tcp/127.0.0.1/$MONPORT"; } 2>/dev/null && break
    sleep 0.2
  done
  [ "$i" -lt 20 ] || fail "cannot reach QEMU monitor on port $MONPORT"
  printf '%s\n' "$@" >&3
  sleep 1 # let QEMU act before we drop the connection
  exec 3>&-
}

# ppm_stats <file> — prints "<zero-bytes> <nonzero-bytes>" of the pixel data.
ppm_stats() {
  local total nz
  head -c 2 "$1" | grep -q '^P6$' || fail "$1 is not a binary PPM"
  total=$(wc -c < "$1" | tr -d ' ')
  nz=$(tr -d '\000' < "$1" | wc -c | tr -d ' ')
  echo "$((total - nz)) $nz"
}
