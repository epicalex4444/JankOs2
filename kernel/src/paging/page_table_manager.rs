use crate::asm;
use crate::efi::EFI_MEMORY_DESCRIPTOR;

//converts linear address to physical address
fn linear_to_physical(linear_address:*const u64) -> *const u64 {
    //linear adress format
    //47:39, pml4 index
    //38:30, pml3 index
    //29:21, pml2 index
    //20:12, pml1 index
    //11:0, page offset

    let pml4_index  = linear_address as isize | 0xFF8000000000;
    let pml3_index  = linear_address as isize |   0x7FD0000000;
    let pml2_index  = linear_address as isize |     0x2FE00000;
    let pml1_index  = linear_address as isize |       0x1FF000;
    let page_offset = linear_address as isize |          0xFFF;

    let pml4:*const u64 = asm::read_cr3() as *const u64;

    unsafe {
        let pml3 = *pml4.offset(pml4_index) as *const u64;
        let pml2 = *pml3.offset(pml3_index) as *const u64;
        let pml1 = *pml2.offset(pml2_index) as *const u64;
        let page = *pml1.offset(pml1_index) as *const u64;
        return *page.offset(page_offset) as *const u64;
    }
}

//maps a physical address to a linear address
fn map_page() -> () {

}

//initialises the page table manager
pub fn init_page_table_manager(
    memory_map: *const EFI_MEMORY_DESCRIPTOR, 
    memory_map_size:u64, 
    descriptor_size:u64
) -> () {

}
