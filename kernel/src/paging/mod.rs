mod page_frame_allocator;
mod page_table_manager;

use crate::efi::EFI_MEMORY_DESCRIPTOR;

//initialises paging
pub fn init_paging(
    memory_map: *const EFI_MEMORY_DESCRIPTOR, 
    memory_map_size:u64, 
    descriptor_size:u64
) -> () {
    //each paging structure is 4096 bytes in size
    //in paging level 4 it has 512 enties with 8 bytes each

    //48 bit linear addresses, to 52 bit physical addresses

    //format of cr3
    //3, page write through
    //4, page level cache disable
    //11:5, ignored
    //51:12, physical address of pml4
    //63:52, reserved must be 0

    //format of pml4, pml3, pml2 and pml1 entries
    //0, present
    //1, read/write
    //2, user/supervisor
    //3, page level write through
    //4, page level cache disable
    //5, accessed
    //6, ignored
    //7, must be 0
    //11:8 ignored
    //52:12 physical address to next level down page
    //62:52 ignored
    //63 execute disable

    //figure out which bits need to be set
    //figure out efi memory map
    //figure out how to make the paging structure

    page_frame_allocator::init_page_frame_allocator(memory_map, memory_map_size, descriptor_size);
    page_table_manager::init_page_table_manager(memory_map, memory_map_size, descriptor_size);
}
