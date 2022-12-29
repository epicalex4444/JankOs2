use crate::efi::EFI_MEMORY_DESCRIPTOR;
use crate::math::RoundMath;

const EFI_CONVENTIONAL_MEMORY:u32 = 7;

static mut BITMAP_START:*const u64 = 0 as *const u64;

//calculates how many pages there are in your system assuming 4096 byte pages
fn total_pages(
    memory_map: *const EFI_MEMORY_DESCRIPTOR, 
    memory_map_size:u64, 
    descriptor_size:u64
) -> u64 {
    let mut pages:u64 = 0;
    for i in 0..memory_map_size / descriptor_size {
        let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
        unsafe {
            pages += (*descriptor).number_of_pages
        }
    }
    pages
}

//reserves continous line of pages
//starts at the page that address is located in
fn reserve_pages(address:u64, pages:u64) -> () {
    let start_page:u64 = address / 4096;

    let start_offset:u64 = start_page / 64;
    let end_offset:u64 = (start_page + pages) / 64;

    let start_mask:u64 = !((1 << (start_page % 64)) - 1);
    let end_mask:u64 = (1 << ((start_page + pages) % 64)) - 1;
    
    unsafe {
        let mem_write:*mut u64 = BITMAP_START.offset(start_offset as isize) as *mut u64;

        if start_offset == end_offset {
            *mem_write |= !start_mask ^ end_mask;
            return
        }

        *mem_write |= start_mask;

        for i in 1..end_offset - start_offset {
            *mem_write.offset(i as isize) = 0xFFFFFFFFFFFFFFFF;
        }

        *mem_write.offset(end_offset as isize) |= end_mask;
    }
}

//frees continous line of pages
//starts at the page that address is located in
//note this can be used to free any memory including things like the kernel
pub fn free_pages(address:u64, pages:u64) -> () {
    let start_page:u64 = address / 4096;

    let start_offset:u64 = start_page / 64;
    let end_offset:u64 = (start_page + pages) / 64;

    let start_mask:u64 = !((1 << (start_page % 64)) - 1);
    let end_mask:u64 = (1 << ((start_page + pages) % 64)) - 1;
    
    unsafe {
        let mem_write:*mut u64 = BITMAP_START.offset(start_offset as isize) as *mut u64;

        if start_offset == end_offset {
            *mem_write &= !(!start_mask ^ end_mask);
            return
        }

        *mem_write &= !start_mask;

        for i in 1..end_offset - start_offset {
            *mem_write.offset(i as isize) = 0;
        }

        *mem_write.offset(end_offset as isize) &= !end_mask;
    }
}

//finds a continous line of pages
//reserves the pages
//then returns the start address of the first page
pub fn request_pages(
    memory_map: *const EFI_MEMORY_DESCRIPTOR, 
    memory_map_size:u64, 
    descriptor_size:u64,
    pages:u64
) -> *const u64 {
    for i in 0..memory_map_size / descriptor_size {
        let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
        unsafe {
            if (*descriptor).r#type == EFI_CONVENTIONAL_MEMORY
                    && (*descriptor).physical_start >= 0x1000000
                    && (*descriptor).number_of_pages >= pages {
                reserve_pages((*descriptor).physical_start, pages);
                return (*descriptor).physical_start as *const u64
            }
        }
    }

    0 as *const u64
}

//initiliase the page frame allocator
//must be called before using request_pages and free_pages
//sets up bitmap that keeps track of which pages are reserved and free
//also parses the memory map and reserves all the memory that we cant use
pub fn init_page_frame_allocator(
    memory_map: *const EFI_MEMORY_DESCRIPTOR, 
    memory_map_size:u64, 
    descriptor_size:u64
) -> () {
    let total_pages:u64 = total_pages(memory_map, memory_map_size, descriptor_size);
    let pages_required:u64 = total_pages.ceil(4096) / 4096;

    for i in 0..memory_map_size / descriptor_size {
        let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
        unsafe {
            if (*descriptor).r#type == EFI_CONVENTIONAL_MEMORY
                    && (*descriptor).physical_start >= 0x1000000
                    && (*descriptor).number_of_pages >= pages_required {
                BITMAP_START = (*descriptor).physical_start as *const u64;
                break;
            }
        }
    }

    unsafe {
        free_pages(0, total_pages);
        reserve_pages(BITMAP_START as u64, pages_required);
    }

    for i in 0..memory_map_size / descriptor_size {
        let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
        unsafe {
            if (*descriptor).r#type != EFI_CONVENTIONAL_MEMORY
                    && (*descriptor).physical_start < 0x1000000 {
                reserve_pages((*descriptor).physical_start, (*descriptor).number_of_pages);
            }
        }
    }
}