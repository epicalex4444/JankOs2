use crate::bitmap;
use crate::efi::EFI_MEMORY_DESCRIPTOR;
use crate::rounding::RoundMath;

pub static mut BITMAP: bitmap::Bitmap = bitmap::Bitmap{bitmap_ptr: core::ptr::null_mut(), length:1};

pub fn init_paging(memory_map: *const EFI_MEMORY_DESCRIPTOR, memory_map_size: u64, descriptor_size: u64) -> bool {
    let mut memory_pages: u64 = 0;
    let memory_map_entries: u64 = memory_map_size / descriptor_size;

    for i in 0..memory_map_entries {
        unsafe {
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
            memory_pages += (*descriptor).number_of_pages;
        }
    }

    let bitmap_pages: u64 = memory_pages.ceil(4096) / 4096;
    let mut bitmap_start: *mut u8 = 0 as *mut u8;
    
    for i in 0..memory_map_entries {
        unsafe {
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
            if (*descriptor).r#type == 7 && (*descriptor).number_of_pages >= bitmap_pages {
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
        BITMAP = bitmap::Bitmap::new(bitmap_start, memory_pages.ceil(8) / 8);
    }

    let mut bitmap_index: u64 = 0;
    for i in 0..memory_map_entries {
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

    return reserve_pages(bitmap_start as u64, bitmap_pages);
}

pub fn free_page(address: u64) -> () {
    let index: u64 = address.ceil(4096) / 4096;
    unsafe {
        BITMAP.clear_bit(index);
    }
}

pub fn free_pages(mut address: u64, pages: u64) -> () {
    for _ in 0..pages {
        free_page(address);
        address += 4096;
    }
}

fn reserve_page(address: u64) -> bool {
    let index: u64 = address.ceil(4096) / 4096;
    unsafe {
        if BITMAP.get_bit(index) {
            return true;
        } else {
            BITMAP.set_bit(index);
            return false;
        }
    }
}

fn reserve_pages(mut address: u64, pages: u64) -> bool {
    for _ in 0..pages {
        if reserve_page(address) {
            return true;
        }
        address += 4096;
    }
    return false;
}

pub fn request_page() -> u64 {
    unsafe {
        for i in 0..BITMAP.length {
            if !BITMAP.get_bit(i) {
                let address: u64 = i * 4096 as u64;
                reserve_page(address);
                return address;
            }
        }
    }
    return 0;
}

pub fn request_pages(pages: u64) -> u64 {
    let mut length: u64 = 0;
    let mut start: u64 = 0;

    unsafe {
        for i in 0..BITMAP.length {
            if length == 0 {
                start = i;
            }

            if length == pages {
                let address: u64 = start * 4096 as u64;
                reserve_pages(address, pages);
                return address;
            }

            if !BITMAP.get_bit(i) {
                length += 1;
            } else {
                length = 0;
            }
        }
    }

    return 0;
}
