#include <efi.h>
#include <efilib.h>
#include <elf.h>

typedef struct {
	uint32_t *BaseAddress;
	uint64_t BufferSize;
	uint32_t Width;
	uint32_t Height;
	uint32_t PixelsPerScanLine;
} Framebuffer;

typedef struct {
	Framebuffer* framebuffer;
	EFI_MEMORY_DESCRIPTOR* mM;
	uint64_t mMSize;
	uint64_t mMDescSize;
	uint8_t* glyphBuffer;
} BootInfo;

//returns the file handle to the volume that the efi file is in
EFI_FILE_HANDLE GetVolume(EFI_HANDLE image) {
	EFI_LOADED_IMAGE *loaded_image = NULL;
	EFI_GUID lipGuid = EFI_LOADED_IMAGE_PROTOCOL_GUID;
	EFI_FILE_IO_INTERFACE *IOVolume;
	EFI_GUID fsGuid = EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID;
	EFI_FILE_HANDLE Volume;
	uefi_call_wrapper(BS->HandleProtocol, 3, image, &lipGuid, (void**)&loaded_image);
	uefi_call_wrapper(BS->HandleProtocol, 3, loaded_image->DeviceHandle, &fsGuid, (VOID*)&IOVolume);
	uefi_call_wrapper(IOVolume->OpenVolume, 2, IOVolume, &Volume);
	return Volume;
}

//returns length of a file in bytes
UINT64 FileSize(EFI_FILE_HANDLE FileHandle) {
	UINT64 ret;
	EFI_FILE_INFO *FileInfo;
	FileInfo = LibFileInfo(FileHandle);
	ret = FileInfo->FileSize;
	FreePool(FileInfo);
	return ret;
}

#define PSF1_MAGIC0	0x36
#define PSF1_MAGIC1	0x04
#define PSF1_MODEHASTAB 0x02

typedef struct {
	uint8_t magic[2];
	uint8_t mode;
	uint8_t charsize;
} PSF1_HEADER;

uint8_t* LoadFont(EFI_FILE_HANDLE font, EFI_SYSTEM_TABLE *SystemTable) {
	PSF1_HEADER psf1_hdr;
	UINTN size = sizeof(PSF1_HEADER);
	uefi_call_wrapper(font->Read, 3, font, &size, &psf1_hdr);

	if (psf1_hdr.magic[0] != PSF1_MAGIC0
			|| psf1_hdr.magic[1] != PSF1_MAGIC1
			|| psf1_hdr.mode != PSF1_MODEHASTAB
			|| psf1_hdr.charsize != 16) {
		Print(L"incorrect font format");
		return NULL;
	}

	UINTN glyphBufferSize = psf1_hdr.charsize * 256;
	uint8_t* glyphBuffer;
	uefi_call_wrapper(font->SetPosition, 2, font, size);
	uefi_call_wrapper(SystemTable->BootServices->AllocatePool, 3, EfiLoaderData, glyphBufferSize, (void**)&glyphBuffer);
	uefi_call_wrapper(font->Read, 3, font, &glyphBufferSize, glyphBuffer);

	return glyphBuffer;
}

EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable) {
	InitializeLib(ImageHandle, SystemTable);

	//get file handle to the kernel
	EFI_FILE_HANDLE Volume = GetVolume(ImageHandle);
	EFI_FILE_HANDLE Kernel;
	uefi_call_wrapper(Volume->Open, 5, Volume, &Kernel, L"kernel", EFI_FILE_MODE_READ, EFI_FILE_READ_ONLY | EFI_FILE_HIDDEN | EFI_FILE_SYSTEM);

	//check kernel exists
	if (Kernel == NULL) {
		Print(L"kernel not found\n");
		return EFI_LOAD_ERROR;
	}

	//read the elf header
	Elf64_Ehdr ehdr;
	UINTN size = sizeof(Elf64_Ehdr);
	uefi_call_wrapper(Kernel->Read, 3, Kernel, &size, &ehdr);

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
		uefi_call_wrapper(Kernel->SetPosition, 2, Kernel, offset);
		uefi_call_wrapper(Kernel->Read, 3, Kernel, &size, &phdr);

		//if the program header says the data is loadable we load it
		if (phdr.p_type == PT_LOAD) {
			//allocate memory for program data
			int pages = (phdr.p_memsz + 0x1000 - 1) / 0x1000;
			Elf64_Addr segment = phdr.p_paddr;
			uefi_call_wrapper(SystemTable->BootServices->AllocatePages, 4, AllocateAddress, EfiLoaderData, pages, &segment);

			//write program data into memory
			uefi_call_wrapper(Kernel->SetPosition, 2, Kernel, phdr.p_offset);
			UINTN size = phdr.p_filesz;
			uefi_call_wrapper(Kernel->Read, 3, Kernel, &size, (void*)segment);
		}

		//point to next program header
		offset += size;
	}

	//get graphics output protocol
	EFI_GUID gopGUID = EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID;
	EFI_GRAPHICS_OUTPUT_PROTOCOL *gop;
	uefi_call_wrapper(SystemTable->BootServices->LocateProtocol, 3, &gopGUID, NULL, (VOID**)&gop);

	//define framebuffer
	Framebuffer framebuffer;
	framebuffer.BaseAddress = (void*)gop->Mode->FrameBufferBase;
	framebuffer.BufferSize = gop->Mode->FrameBufferSize;
	framebuffer.Width = gop->Mode->Info->HorizontalResolution;
	framebuffer.Height = gop->Mode->Info->VerticalResolution;
	framebuffer.PixelsPerScanLine = gop->Mode->Info->PixelsPerScanLine;

	//open font file
	EFI_FILE_HANDLE Font;
	uefi_call_wrapper(Volume->Open, 5, Volume, &Font, L"zap-light16.psf", EFI_FILE_MODE_READ, EFI_FILE_READ_ONLY | EFI_FILE_HIDDEN | EFI_FILE_SYSTEM);
	if (Font == NULL) {
		Print(L"font not found\n");
		return EFI_LOAD_ERROR;
	}

	//load glyphBuffer
	uint8_t* glyphBuffer = LoadFont(Font, SystemTable);
	if (glyphBuffer == NULL) {
		Print(L"error loading glyph buffer");
		return EFI_LOAD_ERROR;
	}

	//get memory map
	UINTN mMSize = 0;
	EFI_MEMORY_DESCRIPTOR* mM = NULL;
	UINTN mMKey;
	UINTN mMDescSize;
	UINT32 mMDescVersion;
	uefi_call_wrapper(SystemTable->BootServices->GetMemoryMap, 5, &mMSize, mM, &mMKey, &mMDescSize, &mMDescVersion);
	uefi_call_wrapper(SystemTable->BootServices->AllocatePool, 3, EfiBootServicesData, mMSize + 2 * mMDescSize, &mM);
	uefi_call_wrapper(SystemTable->BootServices->GetMemoryMap, 5, &mMSize, mM, &mMKey, &mMDescSize, &mMDescVersion);

	//exit boot services
	uefi_call_wrapper(SystemTable->BootServices->ExitBootServices, 2, ImageHandle, mMKey);

	//set virtual addresses manually since SetVirtualAddressMap doesn't fucking work
	for (int i = 0; i < mMSize / mMDescSize; ++i) {
        EFI_MEMORY_DESCRIPTOR* desc = (EFI_MEMORY_DESCRIPTOR*)((uint64_t)mM + (i * mMDescSize));
		void* ptr = (void*)desc->PhysicalStart;
        uefi_call_wrapper(SystemTable->RuntimeServices->ConvertPointer, 2, EFI_OPTIONAL_PTR, &ptr);
		desc->VirtualStart = (EFI_VIRTUAL_ADDRESS)ptr;
    }

	BootInfo bootInfo;
	bootInfo.framebuffer = &framebuffer;
	bootInfo.mM = mM;
	bootInfo.mMSize = mMSize;
	bootInfo.mMDescSize = mMDescSize;
	bootInfo.glyphBuffer = glyphBuffer;

	//define KernelStart function
	uint64_t (*KernelStart)(BootInfo*) = ((__attribute__((sysv_abi)) uint64_t(*)(BootInfo*))ehdr.e_entry);

	//execute kernel
	uint64_t kernel_val = KernelStart(&bootInfo);
	Print(L"kernel return: %lu\n", kernel_val);
	
	return EFI_SUCCESS;
}
