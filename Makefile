BOOTLOADER := bootloader/main.efi
KERNEL := kernel/target/kernel/release/kernel
IMG := JankOs.img
OVMF := /usr/share/ovmf/x64/OVMF.fd
STARTUP := startup.nsh

.PHONY: all qemu clean

all: $(IMG)

$(BOOTLOADER):
	make -C bootloader

$(KERNEL):
	cd kernel && cargo xbuild --release --target kernel.json && cd ..

$(IMG): $(BOOTLOADER) $(STARTUP) $(KERNEL)
	dd if=/dev/zero of=$@ bs=1k count=1440
	mformat -i $@ -f 1440 ::
	mmd -i $@ ::/efi
	mmd -i $@ ::/efi/boot
	mcopy -i $@ $(BOOTLOADER) ::/efi/boot
	mcopy -i $@ $(STARTUP) ::
	mcopy -i $@ $(KERNEL) ::

qemu: $(IMG) $(OVMF)
	qemu-system-x86_64 -drive file=$(IMG),format=raw -bios $(OVMF) -net none

clean:
	rm -f $(IMG)
	cd kernel && cargo clean && cd ..
	make -C bootloader clean
