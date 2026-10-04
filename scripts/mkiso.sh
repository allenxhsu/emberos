#!/usr/bin/env bash
# mkiso.sh <kernel-elf> <out.iso> — wrap a kernel and Limine into a hybrid
# BIOS+UEFI bootable ISO. Expects build/limine (see: make limine).
set -euo pipefail
KERNEL=$1; ISO=$2
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LIMINE="$ROOT/build/limine"
STAGE="$(mktemp -d)"; trap 'rm -rf "$STAGE"' EXIT

mkdir -p "$STAGE/boot/limine" "$STAGE/EFI/BOOT"
cp "$KERNEL" "$STAGE/boot/emberos"
cp "$ROOT/limine.conf" "$LIMINE/limine-bios.sys" "$LIMINE/limine-bios-cd.bin" \
   "$LIMINE/limine-uefi-cd.bin" "$STAGE/boot/limine/"
cp "$LIMINE/BOOTX64.EFI" "$LIMINE/BOOTIA32.EFI" "$STAGE/EFI/BOOT/"

xorriso -as mkisofs -quiet -R -r -J \
  -b boot/limine/limine-bios-cd.bin -no-emul-boot -boot-load-size 4 -boot-info-table \
  -hfsplus -apm-block-size 2048 \
  --efi-boot boot/limine/limine-uefi-cd.bin -efi-boot-part --efi-boot-image \
  --protective-msdos-label "$STAGE" -o "$ISO"
"$LIMINE/limine" bios-install "$ISO" >/dev/null
echo "built $ISO"
