INC_DIR := /usr/local/include/efi
LIB_DIR := /usr/local/lib
LINK_SCRIPT := $(LIB_DIR)/elf_x86_64_efi.lds
CRT0 := $(LIB_DIR)/crt0-efi-x86_64.o

CFLAGS := -fno-stack-protector -fpic -fshort-wchar -mno-red-zone -I$(INC_DIR) -DEFI_FUNCTION_WRAPPER -c
LDFLAGS := -nostdlib -znocombreloc -T$(LINK_SCRIPT) -shared -Bsymbolic -L /usr/local/lib -l:libgnuefi.a -l:libefi.a
OBJFLAGS := -j .text -j .sdata -j .data -j .dynamic -j .dynsym -j .rel -j .rela -j .reloc --target=efi-app-x86_64

.PHONY: all clean

all: main.efi

main.o: main.c
	gcc $(CFLAGS) main.c -o main.o

main.so: $(CRT0) main.o
	ld main.o /usr/local/lib/crt0-efi-x86_64.o $(LDFLAGS) -o main.so

main.efi: main.so
	objcopy $(OBJFLAGS) main.so main.efi

clean:
	rm main.o main.so main.efi
