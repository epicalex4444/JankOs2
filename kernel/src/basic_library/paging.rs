use super::bitmap;
<<<<<<< HEAD
use crate::efi_handover::efi_bindings::EFI_MEMORY_DESCRIPTOR;
=======
use crate::efi_handover::efi_bindings;
use crate::EFI_MEMORY_DESCRIPTOR;
>>>>>>> 98f671a42ef1bb6826a3ca73b90142af6ad384ba
use super::print;

pub static mut BITMAP: bitmap::Bitmap = bitmap::Bitmap{bitmap_ptr: core::ptr::null_mut(), length:1};

pub fn init_paging(memory_map: *const EFI_MEMORY_DESCRIPTOR, memory_map_size: u64, descriptor_size: u64) -> bool {
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
            if (*descriptor).r#type == 7 && (*descriptor).number_of_pages >= memory_pages_pages {
                bitmap_start = (*descriptor).physical_start as *mut u8;
                break;
            }
        }
    }

    if bitmap_start == 0 as *mut u8 {
        return true;
    }

    //init bitmap
    unsafe {
        BITMAP  = bitmap::Bitmap::new(bitmap_start, memory_pages / 8 + 1);
    }

    let mut bitmap_index: u64 = 0;
    for i in 0..memory_map_size / descriptor_size {
        unsafe {
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
            if (*descriptor).r#type == 7 {
                for _ in 0..(*descriptor).number_of_pages {
                    BITMAP.clear_bit(bitmap_index);
                    bitmap_index += 1;
                }
            } else {
                for _ in 0..(*descriptor).number_of_pages {
                    BITMAP.set_bit(bitmap_index);
                    bitmap_index += 1;
                }
            }
        }
    }

    return reserve_pages(bitmap_start as u64, memory_pages_pages);
}

pub fn free_page(address: u64) -> () {
    let index: u64 = address / 4096;
    unsafe {
        BITMAP.clear_bit(index);
    }
}

pub fn free_pages(address: u64, pages: u64) -> () {
    for _ in 0..pages {
        free_page(address);
    }
}

pub fn reserve_page(address: u64) -> bool {
    let index: u64 = address / 4096;
    unsafe{
        if BITMAP.get_bit(index){
            return true;
        } else {
            BITMAP.set_bit(index);
            return false;
        }
    }
}

pub fn reserve_pages(address: u64, pages: u64) -> bool {
    for _ in 0..pages {
        if reserve_page(address) {
            return true;
        }
    }
    return false;
}