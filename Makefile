ARCH = x86_64
TARGET = helloworld.efi
SRCS = $(wildcard *.c)
CFLAGS = -pedantic -Wall -Wextra -std=gnu17 -O2

helloworld: all
	mv helloworld.efi root/efi/boot

qemu: helloworld
	qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=fat:rw:root -net none

include uefi/Makefile
