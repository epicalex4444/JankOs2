#include <efi.h>
#include <efilib.h>
#include <elf.h>

typedef struct {
	uint32_t *base_address;
	uint64_t buffer_size;
	uint32_t width;
	uint32_t height;
	uint32_t pixels_per_scan_line;
} FrameBuffer;

typedef struct {
	FrameBuffer* frame_buffer;
	EFI_MEMORY_DESCRIPTOR* memory_map;
	uint64_t memory_map_size;
	uint64_t descriptor_size;
	uint8_t* glyph_buffer;
} BootInfo;

//returns the file handle to the volume that the efi file is in
EFI_FILE_HANDLE get_volume(EFI_HANDLE image) {
	EFI_LOADED_IMAGE *loaded_image = NULL;
	EFI_GUID image_guid = EFI_LOADED_IMAGE_PROTOCOL_GUID;
	EFI_FILE_IO_INTERFACE *io;
	EFI_GUID fs_guid = EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID;
	EFI_FILE_HANDLE volume;
	uefi_call_wrapper(BS->HandleProtocol, 3, image, &image_guid, (void**)&loaded_image);
	uefi_call_wrapper(BS->HandleProtocol, 3, loaded_image->DeviceHandle, &fs_guid, (VOID*)&io);
	uefi_call_wrapper(io->OpenVolume, 2, io, &volume);
	return volume;
}

//returns length of a file in bytes
UINT64 file_length(EFI_FILE_HANDLE file_handle) {
	UINT64 length;
	EFI_FILE_INFO *file_info;
	file_info = LibFileInfo(file_handle);
	length = file_info->FileSize;
	FreePool(file_info);
	return length;
}

#define PSF1_MAGIC0	0x36
#define PSF1_MAGIC1	0x04
#define PSF1_MODEHASTAB 0x02

typedef struct {
	uint8_t magic[2];
	uint8_t mode;
	uint8_t charsize;
} Psf1Header;

uint8_t* load_font(EFI_FILE_HANDLE font) {
	Psf1Header psf1_hdr;
	UINTN size = sizeof(Psf1Header);
	uefi_call_wrapper(font->Read, 3, font, &size, &psf1_hdr);

	if (psf1_hdr.magic[0] != PSF1_MAGIC0
			|| psf1_hdr.magic[1] != PSF1_MAGIC1
			|| psf1_hdr.mode != PSF1_MODEHASTAB
			|| psf1_hdr.charsize != 16) {
		Print(L"incorrect font format\n");
		return NULL;
	}

	UINTN glyph_buffer_size = psf1_hdr.charsize * 256;
	uint8_t* glyph_buffer;
	uefi_call_wrapper(font->SetPosition, 2, font, size);
	uefi_call_wrapper(BS->AllocatePool, 3, EfiLoaderData, glyph_buffer_size, (void**)&glyph_buffer);
	uefi_call_wrapper(font->Read, 3, font, &glyph_buffer_size, glyph_buffer);

	return glyph_buffer;
}

EFI_STATUS EFIAPI efi_main(EFI_HANDLE image_handle, EFI_SYSTEM_TABLE *system_table) {
	InitializeLib(image_handle, system_table);

	//get file handle to the kernel
	EFI_FILE_HANDLE volume = get_volume(image_handle);
	EFI_FILE_HANDLE kernel;
	uefi_call_wrapper(volume->Open, 5, volume, &kernel, L"kernel", EFI_FILE_MODE_READ, EFI_FILE_READ_ONLY | EFI_FILE_HIDDEN | EFI_FILE_SYSTEM);

	//check kernel exists
	if (kernel == NULL) {
		Print(L"kernel not found\n");
		return EFI_LOAD_ERROR;
	}

	//read the elf header
	Elf64_Ehdr ehdr;
	UINTN size = sizeof(Elf64_Ehdr);
	uefi_call_wrapper(kernel->Read, 3, kernel, &size, &ehdr);

	//check the elf header
	if (ehdr.e_ident[EI_MAG0] != 0x7F
			|| ehdr.e_ident[EI_MAG1] != 'E'
			|| ehdr.e_ident[EI_MAG2] != 'L'
			|| ehdr.e_ident[EI_MAG3] != 'F'         //is it an elf file
			|| ehdr.e_ident[EI_CLASS] != ELFCLASS64 //is it 64 bit
			|| ehdr.e_ident[EI_DATA] != ELFDATA2LSB //is it little endian
			|| ehdr.e_type != ET_EXEC               //is it executable
			|| ehdr.e_machine != EM_X86_64          //is it x86_64
			|| ehdr.e_version != EV_CURRENT) {      //is it the current elf version
		Print(L"kernel is not the correct format\n");
		return EFI_LOAD_ERROR;
	}

	//read program headers
	uint64_t offset = ehdr.e_phoff;
	for (uint16_t i = 0; i < ehdr.e_phnum; ++i) {
		//read program header
		Elf64_Phdr phdr;
		UINTN size = sizeof(Elf64_Phdr);
		uefi_call_wrapper(kernel->SetPosition, 2, kernel, offset);
		uefi_call_wrapper(kernel->Read, 3, kernel, &size, &phdr);

		//if the program header says the data is loadable we load it
		if (phdr.p_type == PT_LOAD) {
			//allocate memory for program data
			int pages = (phdr.p_memsz + 0x1000 - 1) / 0x1000;
			Elf64_Addr segment = phdr.p_paddr;
			uefi_call_wrapper(BS->AllocatePages, 4, AllocateAddress, EfiLoaderData, pages, &segment);

			//write program data into memory
			uefi_call_wrapper(kernel->SetPosition, 2, kernel, phdr.p_offset);
			UINTN size = phdr.p_filesz;
			uefi_call_wrapper(kernel->Read, 3, kernel, &size, (void*)segment);
		}

		//point to next program header
		offset += size;
	}

	//get graphics output protocol
	EFI_GUID gop_guid = EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
	EFI_GRAPHICS_OUTPUT_PROTOCOL *gop;
	uefi_call_wrapper(BS->LocateProtocol, 3, &gop_guid, NULL, (VOID**)&gop);

	//define framebuffer
	FrameBuffer frame_buffer;
	frame_buffer.base_address = (void*)gop->Mode->FrameBufferBase;
	frame_buffer.buffer_size = gop->Mode->FrameBufferSize;
	frame_buffer.width = gop->Mode->Info->HorizontalResolution;
	frame_buffer.height = gop->Mode->Info->VerticalResolution;
	frame_buffer.pixels_per_scan_line = gop->Mode->Info->PixelsPerScanLine;

	//open font file
	EFI_FILE_HANDLE font;
	uefi_call_wrapper(volume->Open, 5, volume, &font, L"zap-light16.psf", EFI_FILE_MODE_READ, EFI_FILE_READ_ONLY | EFI_FILE_HIDDEN | EFI_FILE_SYSTEM);
	if (font == NULL) {
		Print(L"font not found\n");
		return EFI_LOAD_ERROR;
	}

	//load glyphBuffer
	uint8_t* glyph_buffer = load_font(font);
	if (glyph_buffer == NULL) {
		Print(L"error loading glyph buffer\n");
		return EFI_LOAD_ERROR;
	}

	//get memory map
	UINTN memory_map_size = 0;
	EFI_MEMORY_DESCRIPTOR* memory_map = NULL;
	UINTN memory_map_key;
	UINTN descriptor_size;
	UINT32 descriptor_version;
	uefi_call_wrapper(BS->GetMemoryMap, 5, &memory_map_size, memory_map, &memory_map_key, &descriptor_size, &descriptor_version);
	uefi_call_wrapper(BS->AllocatePool, 3, EfiBootServicesData, memory_map_size + 2 * descriptor_size, &memory_map);
	uefi_call_wrapper(BS->GetMemoryMap, 5, &memory_map_size, memory_map, &memory_map_key, &descriptor_size, &descriptor_version);

	//exit boot services
	uefi_call_wrapper(BS->ExitBootServices, 2, image_handle, memory_map_key);

	BootInfo boot_info;
	boot_info.frame_buffer = &frame_buffer;
	boot_info.memory_map = memory_map;
	boot_info.memory_map_size = memory_map_size;
	boot_info.descriptor_size = descriptor_size;
	boot_info.glyph_buffer = glyph_buffer;

	//define KernelStart function
	void (*KernelStart)(BootInfo*) = ((__attribute__((sysv_abi)) void(*)(BootInfo*))ehdr.e_entry);

	//execute kernel
	KernelStart(&boot_info);
	
	return EFI_SUCCESS;
}