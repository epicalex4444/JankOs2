#include <efi.h>
#include <efilib.h>
#include <elf.h>

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

UINT64 FileSize(EFI_FILE_HANDLE FileHandle) {
	UINT64 ret;
	EFI_FILE_INFO *FileInfo;
	FileInfo = LibFileInfo(FileHandle);
	ret = FileInfo->FileSize;
	FreePool(FileInfo);
	return ret;
}

int memcmp(const void *aptr, const void *bptr, unsigned long n){
	const unsigned char *a = aptr, *b = bptr;
	for (unsigned long i = 0; i < n; i++){
		if (a[i] < b[i]) {
			return -1;
		} else if (a[i] > b[i]) {
			return 1;
		}
	}
	return 0;
}

EFI_STATUS EFIAPI efi_main(EFI_HANDLE ImageHandle, EFI_SYSTEM_TABLE *SystemTable) {
	InitializeLib(ImageHandle, SystemTable);
	Print(L"Hello, world!\n");

	EFI_FILE_HANDLE Volume = GetVolume(ImageHandle);
	CHAR16 *FileName = L"kernel";
	EFI_FILE_HANDLE Kernel;
	uefi_call_wrapper(Volume->Open, 5, Volume, &Kernel, FileName, EFI_FILE_MODE_READ, EFI_FILE_READ_ONLY | EFI_FILE_HIDDEN | EFI_FILE_SYSTEM);

	UINT64 FileInfoSize = FileSize(Kernel);

	Elf64_Ehdr header;
	UINTN size = sizeof(header);
	uefi_call_wrapper(Kernel->Read, 3, Kernel, &size, &header);

	Print(L"header.e_ident = [%hhu", header.e_ident[0]);
	for (int i = 1; i < 16; ++i) {
		Print(L", %hhu", header.e_ident[i]);
	}
	Print(L"]\n");
	Print(L"header.e_type = %hu\n", header.e_type);
	Print(L"header.e_machine = %hu\n", header.e_machine);
	Print(L"header.e_version = %u\n", header.e_version);
	Print(L"header.e_entry = %lu\n", header.e_entry);
	Print(L"header.e_phoff = %lu\n", header.e_phoff);
	Print(L"header.e_shoff = %lu\n", header.e_shoff);
	Print(L"header.e_flags = %u\n", header.e_flags);
	Print(L"header.e_ehsize = %hu\n", header.e_ehsize);
	Print(L"header.e_phentsize = %hu\n", header.e_phentsize);
	Print(L"header.e_phnum = %hu\n", header.e_phnum);
	Print(L"header.e_shentsize = %hu\n", header.e_shentsize);
	Print(L"header.e_shnum = %hu\n", header.e_shnum);
	Print(L"header.e_shstrndx = %hu\n", header.e_shstrndx);

	Print(L"Goodbye World!\n");
	return EFI_SUCCESS;
}
