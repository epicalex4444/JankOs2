BOOTLOADER := bootloader/bootx64.efi
BOOTLOADER_DEPS := bootloader/bootx64.c
KERNEL := kernel/target/x86_64-kernel/release/kernel
KERNEL_DEPS := $(shell find -path "./kernel/src/*.rs") $(shell find -path "./kernel/src/*.asm")
IMG := JankOS.img
OVMF := /usr/share/ovmf/x64/OVMF.fd
FONT := zap-light16.psf

.PHONY: all run.img qemu qemu_debug clean

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

run.img: $(BOOTLOADER) $(FONT)
	@echo testing binary at $(filter-out $@,$(MAKECMDGOALS))
	cp $(filter-out $@,$(MAKECMDGOALS)) kernel/kernel
	dd if=/dev/zero of=$@.img bs=1k count=2880
	mformat -i $@ -f 2880 ::
	mmd -i $@ ::/efi
	mmd -i $@ ::/efi/boot
	mcopy -i $@ $(BOOTLOADER) ::/efi/boot
	mcopy -i $@ kernel/kernel ::
	mcopy -i $@ $(FONT) ::
	qemu-system-x86_64 -drive file=run.img,format=raw -bios $(OVMF) -net none
%:
	@:

qemu: $(IMG) $(OVMF)
	qemu-system-x86_64 -drive file=$(IMG),format=raw -bios $(OVMF) -net none

qemu_debug: $(IMG) $(OVMF)
	qemu-system-x86_64 -drive file=$(IMG),format=raw -bios $(OVMF) -net none -monitor stdio -d cpu_reset 

clean:
	rm -f $(IMG)
	cd kernel && cargo clean && cd ..
	make -C bootloader clean
