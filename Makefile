BOOTLOADER := bootloader/main.efi
IMG := JankOs.img
OVMF := /usr/share/ovmf/x64/OVMF.fd
STARTUP := startup.nsh

.PHONY: all qemu clean

all: $(IMG)

$(BOOTLOADER):
	make -C bootloader

$(IMG): $(BOOTLOADER)
	dd if=/dev/zero of=$@ bs=1k count=1440
	mformat -i $@ -f 1440 ::
	mmd -i $@ ::/efi
	mmd -i $@ ::/efi/boot
	mcopy -i $@ $(BOOTLOADER) ::/efi/boot
	mcopy -i $@ $(STARTUP) ::

qemu: $(IMG)
	qemu-system-x86_64 -drive file=$(IMG),format=raw -drive if=pflash,format=raw,unit=0,readonly=on,file=$(OVMF) -net none

clean:
	rm $(IMG)
	make -C bootloader clean
