SHELL := /bin/bash
TARGET := x86_64-unknown-none
PROFILE ?= release
PROFILE_DIR := $(if $(filter release,$(PROFILE)),release,debug)
KERNEL := target/$(TARGET)/$(PROFILE_DIR)/emberos
LIMINE_DIR := build/limine
LIMINE_URL := https://github.com/limine-bootloader/limine.git
LIMINE_BRANCH := v9.x-binary
QEMU := qemu-system-x86_64 -M q35 -m 128M -boot d -vga std \
        -device isa-debug-exit,iobase=0xf4,iosize=0x04 -no-reboot

.PHONY: all toolchain limine kernel iso run test clean

all: iso

toolchain:
	rustup target add $(TARGET)

$(LIMINE_DIR)/limine:
	git clone --depth=1 --branch=$(LIMINE_BRANCH) $(LIMINE_URL) $(LIMINE_DIR)
	$(MAKE) -C $(LIMINE_DIR)
limine: $(LIMINE_DIR)/limine

kernel:
	cargo build --profile $(PROFILE)

iso: kernel limine
	mkdir -p build
	scripts/mkiso.sh $(KERNEL) build/emberos.iso

build/emberos-panic.iso: limine
	cargo build --profile $(PROFILE) --features selftest-panic --target-dir target/panic
	mkdir -p build
	scripts/mkiso.sh target/panic/$(TARGET)/$(PROFILE_DIR)/emberos $@

run: iso
	$(QEMU) -cdrom build/emberos.iso -serial stdio -rtc base=localtime

test: iso build/emberos-panic.iso
	tests/boot.sh
	tests/panic.sh

clean:
	cargo clean
	rm -rf build
