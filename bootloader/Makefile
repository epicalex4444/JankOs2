INC_DIR := /usr/include/efi
LIB_DIR := /usr/lib
LINK_SCRIPT := $(LIB_DIR)/elf_x86_64_efi.lds
CRT0 := $(LIB_DIR)/crt0-efi-x86_64.o

CFLAGS := -fno-stack-protector -fpic -fshort-wchar -mno-red-zone -I$(INC_DIR) -DEFI_FUNCTION_WRAPPER -c
LDFLAGS := -nostdlib -znocombreloc -T$(LINK_SCRIPT) -shared -Bsymbolic -L$(LIB_DIR) -l:libgnuefi.a -l:libefi.a
OBJFLAGS := -j .text -j .sdata -j .data -j .dynamic -j .dynsym -j .rel -j .rela -j .reloc --target=efi-app-x86_64

.PHONY: all clean

all: main.efi

main.o: main.c
	gcc $(CFLAGS) $< -o $@

main.so: $(CRT0) main.o
	ld $^ $(LDFLAGS) -o $@

main.efi: main.so
	objcopy $(OBJFLAGS) $< $@
	
clean:
	rm -f main.o main.so main.efi
