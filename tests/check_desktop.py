#!/usr/bin/env python3
"""Assert that a QEMU screendump (binary PPM) shows the EmberOS desktop.

Geometry comes from the UI concept in README.md ("Desktop"), expressed
relative to the screen size so any resolution works.
"""
import sys

DESKTOP, FACE = (0, 128, 128), (192, 192, 192)
WHITE, LIGHT, SHADOW, BLACK = (255, 255, 255), (223, 223, 223), (128, 128, 128), (0, 0, 0)
TITLE = (0, 0, 128)
TASKBAR_H, WIN_W, WIN_H = 28, 420, 200
ICON_LABELS = 3


def load(path):
    data = open(path, "rb").read()
    fields, i = [], 0
    while len(fields) < 4:
        while data[i:i + 1].isspace():
            i += 1
        j = i
        while not data[j:j + 1].isspace():
            j += 1
        fields.append(data[i:j])
        i = j
    if fields[0] != b"P6" or fields[3] != b"255":
        sys.exit(f"FAIL: {path} is not an 8-bit binary PPM")
    w, h = int(fields[1]), int(fields[2])
    px = data[i + 1:]
    if len(px) != w * h * 3:
        sys.exit(f"FAIL: {path} is truncated")
    return w, h, px


W, H, PX = load(sys.argv[1])
failures = []


def at(x, y):
    o = (y * W + x) * 3
    return tuple(PX[o:o + 3])


def count(colour, x, y, w, h):
    return sum(at(i, j) == colour for j in range(y, y + h) for i in range(x, x + w))


def expect(name, ok, detail=""):
    if not ok:
        failures.append(f"{name} {detail}".strip())


def expect_px(name, x, y, colour):
    expect(name, at(x, y) == colour, f"at ({x},{y}): want {colour}, got {at(x, y)}")


def expect_count(name, colour, rect, least):
    n = count(colour, *rect)
    expect(name, n >= least, f"in {rect}: want >= {least} px of {colour}, got {n}")


# Desktop background: corners of the work area that hold no icon or window.
top = H - TASKBAR_H
for x, y in [(W - 8, 8), (W - 8, top - 8), (W // 2, 8), (8, top - 8)]:
    expect_px("desktop background", x, y, DESKTOP)

# Taskbar: grey face, white highlight on its second row.
expect_px("taskbar top row", W // 2, top, FACE)
expect_px("taskbar highlight", W // 2, top + 1, WHITE)
expect_px("taskbar face", W // 2, top + 14, FACE)
expect_px("taskbar bottom", W // 2, H - 1, FACE)

# Start button: raised bevel at (2, H-24), 22 high; width found by scanning.
sx, sy, sh = 2, H - 24, 22
expect_px("start top edge", sx + 5, sy, WHITE)
expect_px("start left edge", sx, sy + 5, WHITE)
expect_px("start bottom edge", sx + 5, sy + sh - 1, BLACK)
sw = 0
while sx + sw < W and at(sx + sw, sy + sh - 1) == BLACK:
    sw += 1
expect("start width", 50 <= sw <= 100, f"want 50..100, got {sw}")
expect_px("start right edge", sx + sw - 1, sy + 5, BLACK)
expect_count("start label", BLACK, (sx + 20, sy + 2, max(sw - 22, 1), sh - 4), 30)
expect("start icon", (sh - 4) * 18 - count(FACE, sx + 3, sy + 2, 18, sh - 4) >= 30, "no icon pixels")

# Tray clock: sunken box ending at x = W-3, same rows as the Start button.
expect_px("clock top edge", W - 12, sy, SHADOW)
expect_px("clock bottom edge", W - 12, sy + sh - 1, WHITE)
expect_px("clock right edge", W - 3, sy + 5, WHITE)
expect_count("clock text", BLACK, (W - 90, sy + 2, 86, sh - 4), 40)

# Desktop icons: 32x32 art at (38, 16 + 76*i), white label centred below.
for i in range(ICON_LABELS):
    iy = 16 + 76 * i
    art = 32 * 32 - count(DESKTOP, 38, iy, 32, 32)
    expect(f"icon {i} art", art >= 200, f"only {art} non-background px")
    expect_count(f"icon {i} label", WHITE, (0, iy + 34, 110, 20), 30)

# Welcome window: centred in the work area.
wx, wy = (W - WIN_W) // 2, (top - WIN_H) // 2
expect_px("window outer top", wx + 10, wy, LIGHT)
expect_px("window inner top", wx + 10, wy + 1, WHITE)
expect_px("window inner bottom", wx + 10, wy + WIN_H - 2, SHADOW)
expect_px("window outer bottom", wx + 10, wy + WIN_H - 1, BLACK)
expect_px("window outer right", wx + WIN_W - 1, wy + 10, BLACK)
expect_px("window face", wx + 10, wy + 100, FACE)
title = (wx + 4, wy + 4, WIN_W - 8, 18)
expect_count("title bar", TITLE, title, title[2] * title[3] // 2)
expect_count("title text", WHITE, (wx + 24, wy + 4, 250, 18), 60)
expect_count("title buttons", FACE, (wx + WIN_W - 60, wy + 6, 54, 14), 200)
expect_count("close glyph", BLACK, (wx + WIN_W - 20, wy + 8, 11, 9), 10)
expect_count("window text", BLACK, (wx + 8, wy + 28, WIN_W - 16, 110), 150)
# OK button: 75x23, default (black ring), centred near the bottom.
bx, by = wx + (WIN_W - 75) // 2, wy + WIN_H - 39
expect_px("ok ring", bx + 10, by, BLACK)
expect_px("ok top edge", bx + 10, by + 1, WHITE)
expect_count("ok label", BLACK, (bx + 20, by + 3, 35, 17), 10)

if failures:
    print(f"FAIL: desktop ({W}x{H}):")
    for f in failures:
        print("  -", f)
    sys.exit(1)
print(f"desktop ok ({W}x{H})")
