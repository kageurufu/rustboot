LD := x86_64-efi-pe-ld
RUSTC := rustc

.phony: all clean build image

all: clean build image

clean:
	rm -r build img || true

build:
	mkdir -p build
	$(RUSTC) -O --emit=obj --crate-type=lib src/boot.rs --out-dir build/
	mkdir -p img/efi/boot
	$(LD) --subsystem 10 -pie -e efi_start build/boot.o -o img/efi/boot/bootx64.efi

image:
	mkisofs -o boot.iso img
