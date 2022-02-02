#include <efi.h>
#include <efilib.h>
#include <elf.h>

typedef struct {
	uint32_t* BaseAddress;
	uint64_t BufferSize;
	uint32_t Width;
	uint32_t Height;
	uint32_t PixelsPerScanLine;
} Framebuffer;

typedef struct {
	Framebuffer *framebuffer;
	EFI_MEMORY_DESCRIPTOR *mM;
	UINTN mMSize;
	UINTN mMDescSize;
} BootInfo;

//returns the gile handle to the volume that the efi file is in
EFI_FILE_HANDLE GetVolume(EFI_HANDLE image) {
	EFI_LOADED_IMAGE *loaded_image = NULL;
	EFI_GUID lipGuid = EFI_LOADED_IMAGE_PROTOCOL_GUID;
	EFI_FILE_IO_INTERFACE *IOVolume;
	EFI_GUID fsGuid = EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID;
	EFI_FILE_HANDLE Volume;
	uefi_call_wrapper(BS->HandleProtocol, 3, image, &lipGuid, (void **) &loaded_image);
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

void PlotPixel(uint32_t x, uint32_t y, uint8_t r, uint8_t g, uint8_t b, Framebuffer framebuffer) {
   *((uint32_t*)(framebuffer.BaseAddress + 4 * framebuffer.PixelsPerScanLine * y + 4 * x)) = (r << 16) + (g << 8) + b;
}

EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable) {
	InitializeLib(ImageHandle, SystemTable);

	//get file handle to the kernel
	EFI_FILE_HANDLE Volume = GetVolume(ImageHandle);
	CHAR16 *FileName = L"kernel";
	EFI_FILE_HANDLE Kernel;
	uefi_call_wrapper(Volume->Open, 5, Volume, &Kernel, FileName, EFI_FILE_MODE_READ, EFI_FILE_READ_ONLY | EFI_FILE_HIDDEN | EFI_FILE_SYSTEM);

	//check kernel exists
	if (Kernel == NULL) {
		Print(L"kernel not found\n");
		return EFI_LOAD_ERROR;
	}

	//read the elf header
	Elf64_Ehdr ehdr;
	UINTN size = sizeof(ehdr);
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
		UINTN size = sizeof(phdr);
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

	//plot green pixel using the framebuffer
	PlotPixel(0, 0, 0, 255, 0, framebuffer);
	Print(L"Plotted Pixel \n");
	//get memory map
	UINTN memoryMapSize = 0;
	EFI_MEMORY_DESCRIPTOR* memoryMap = NULL;
	UINTN mapKey;
	UINTN descriptorSize;
	UINT32 descrptorVersion;
	uefi_call_wrapper(SystemTable->BootServices->GetMemoryMap, 5, &memoryMapSize, memoryMap, &mapKey, &descriptorSize, &descrptorVersion);
	uefi_call_wrapper(SystemTable->BootServices->AllocatePool, 3, EfiBootServicesData, memoryMapSize + 2 * descriptorSize, &memoryMap);
	uefi_call_wrapper(SystemTable->BootServices->GetMemoryMap, 5, &memoryMapSize, memoryMap, &mapKey, &descriptorSize, &descrptorVersion);

	//exit boot services
	//uefi_call_wrapper(SystemTable->BootServices->ExitBootServices, 2, ImageHandle, mapKey);

	//execute kernel
	uint32_t (*KernelStart)(BootInfo*) = ((__attribute__((sysv_abi)) uint32_t (*)(BootInfo*)) ehdr.e_entry);

	BootInfo bootInfo;
	bootInfo.framebuffer = &framebuffer;
	bootInfo.mM = memoryMap;
	bootInfo.mMSize = memoryMapSize;
	bootInfo.mMDescSize = descriptorSize;

	uint32_t moist= KernelStart(&bootInfo);
	Print(L"Kernel returns: %u \n",moist);
	return EFI_SUCCESS;
}
