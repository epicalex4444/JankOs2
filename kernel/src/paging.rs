use crate::{asm, println};
use crate::math::*;
use crate::efi::EFI_MEMORY_DESCRIPTOR;

const EFI_CONVENTIONAL_MEMORY:u32 = 7;

static mut BITMAP_START:*const u64 = 0 as *const u64;

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

fn free_pages(address:u64, pages:u64) -> () {
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

fn request_pages(
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

fn init_page_frame_allocator(
    memory_map: *const EFI_MEMORY_DESCRIPTOR, 
    memory_map_size:u64, 
    descriptor_size:u64
) -> () {
    let total_pages:u64 = total_pages(memory_map, memory_map_size, descriptor_size);
    let pages_required:u64 = total_pages.ceil(4096) / 4096;

    println!("total_pages: {}\npages_required: {}", total_pages, pages_required);

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
        println!("BITMAP_START: {:#0x}", BITMAP_START as u64);
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

pub fn init_paging(
    memory_map: *const EFI_MEMORY_DESCRIPTOR, 
    memory_map_size:u64, 
    descriptor_size:u64
) -> () {
    //each paging structure is 4096 bytes in size
    //in paging level 4 it has 512 enties with 8 bytes each

    //linear adress format
    //47:39, pml4 index
    //38:30, pml3 index
    //29:21, pml2 index
    //20:12, pml1 index
    //12:0, page offset

    //48 bit linear addresses, to 52 bit physical addresses

    //format of cr3
    //3, page write through
    //4, page level cache disable
    //11:5, ignored
    //51:12, physical address of pml4
    //63:52, reserved must be 0

    //format of pml4, pml3, pml2 and pml1 entries
    //0, must be 1
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

    init_page_frame_allocator(memory_map, memory_map_size, descriptor_size);

    unsafe {
        let mem_write:*mut u64 = BITMAP_START as *mut u64;

        for i in 0..10 {
            println!("{:#0b}", *mem_write.offset(i));
        }
        println!("");

        reserve_pages(8192, 256);

        for i in 0..10 {
            println!("{:#0b}", *mem_write.offset(i));
        }
        println!("");
    }
}
