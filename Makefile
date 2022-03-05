BOOTLOADER := bootloader/bootx64.efi
BOOTLOADER_DEPS := bootloader/bootx64.c
KERNEL := kernel/target/x86_64-kernel/release/kernel
KERNEL_DEPS := $(shell find -path "./kernel/src/*.rs")
IMG := JankOs.img
OVMF := /usr/share/ovmf/x64/OVMF.fd
FONT := zap-light16.psf

.PHONY: all qemu qemu_debug clean

all: $(IMG)

$(BOOTLOADER): $(BOOTLOADER_DEPS)
	make -C bootloader

$(KERNEL): $(KERNEL_DEPS)
	cd kernel && cargo build --release --target x86_64-kernel.json && cd ..

$(IMG): $(BOOTLOADER) $(KERNEL) $(FONT)
	dd if=/dev/zero of=$@ bs=1k count=1440
	mformat -i $@ -f 1440 ::
	mmd -i $@ ::/efi
	mmd -i $@ ::/efi/boot
	mcopy -i $@ $(BOOTLOADER) ::/efi/boot
	mcopy -i $@ $(KERNEL) ::
	mcopy -i $@ $(FONT) ::

qemu: $(IMG) $(OVMF)
	qemu-system-x86_64 -drive file=$(IMG),format=raw -bios $(OVMF) -net none

qemu_debug: $(IMG) $(OVMF)
	qemu-system-x86_64 -drive file=$(IMG),format=raw -bios $(OVMF) -net none -monitor stdio -d cpu_reset 

clean:
	rm -f $(IMG)
	cd kernel && cargo clean && cd ..
	make -C bootloader clean
