ARCH = x86_64
TARGET = helloworld.efi
SRCS = $(wildcard *.c)
CFLAGS = -pedantic -Wall -Wextra -Werror --ansi -O2

helloworld: all
	mv helloworld.efi root/efi/boot

qemu: helloworld
	qemu-system-x86_64 -bios /usr/share/ovmf/x64/OVMF.fd -drive format=raw,file=fat:rw:root -net none

include uefi/Makefile
