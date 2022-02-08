use super::bitmap;
use crate::efi_bindings::EFI_MEMORY_DESCRIPTOR;

pub fn init_paging(memory_map: *const EFI_MEMORY_DESCRIPTOR, memory_map_size: u64, descriptor_size: u64) -> () {
    let mut memory_pages: u64 = 0;
    for i in 0..memory_map_size / descriptor_size {
        unsafe {
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
            memory_pages += (*descriptor).number_of_pages;
        }
    }

    let memory_pages_pages: u64 = memory_pages / 4096 + 1;
    let mut bitmap_start: *mut u8 = 0 as *mut u8;
    
    for i in 0..memory_map_size / descriptor_size {
        unsafe {
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
            if (*descriptor).t == 7 && (*descriptor).number_of_pages >= memory_pages_pages {
                bitmap_start = (*descriptor).physical_start as *mut u8;
                break;
            }
        }
    }

    if bitmap_start == 0 as *mut u8 {
        return;
    }

    let bitmap = bitmap::Bitmap::new(bitmap_start, memory_pages / 8 + 1);

    for i in 0..memory_map_size / descriptor_size {
        unsafe {
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
            if (*descriptor).t == 7  {
                //clear bit in bitmap
            } else {
                //set bit in bitmap
            }
        }
    }

    reserve_pages(bitmap as u64, memory_pages_pages);
}

pub fn free_page(address: u64) -> () {
    let index: u64 = address / 4096;
    //clear bitmap index
}

pub fn reserve_page(address: u64) -> () {
    let index: u64 = address / 4096;
    //set bitmap index
}

pub fn free_pages(address: u64, pages: u64) -> () {
    for i in 0..pages {
        free_page(address)
    }
}

pub fn reserve_pages(address: u64, pages: u64) -> () {
    for i in 0..pages {
        reserve_page(address)
    }
}