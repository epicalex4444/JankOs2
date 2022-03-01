use crate::asm;
use crate::efi::EFI_MEMORY_DESCRIPTOR;
use crate::gdt;

macro_rules! EFI_CONVENTIONAL_MEMORY {
    () => {7};
}

pub fn init_paging(
    memory_map: *const EFI_MEMORY_DESCRIPTOR,
    memory_map_size: u64,
    descriptor_size: u64,
) -> () {
    //clear cr4.pcide, must be done to disable paging
    let cr4 = asm::read_cr4();
    asm::write_cr4(cr4 & !(1 << 17));

    //compatability mode must be entered to disable paging
    gdt::enter_compatibility_mode();

    //disable paging
    let cr0 = asm::read_cr0();
    asm::write_cr0(cr0 & !(1 << 31));

    let mut l4_start:*mut u64 = core::ptr::null_mut();
    let mut l3_start:*mut u64 = core::ptr::null_mut();
    let mut l2_start:*mut u64 = core::ptr::null_mut();
    let mut l1_start:*mut u64 = core::ptr::null_mut();

    let mut l4_i: u64 = 0;
    let mut l3_i: u64 = 0;
    let mut l2_i: u64 = 0;
    let mut l1_i: u64 = 0;

    //iterate through efi memory map
    for i in 0..memory_map_size / descriptor_size {
        unsafe {
            let descriptor: *const EFI_MEMORY_DESCRIPTOR = (memory_map as u64 + i * descriptor_size) as *const EFI_MEMORY_DESCRIPTOR;
            
            //if memory is not free
            if (*descriptor).r#type != EFI_CONVENTIONAL_MEMORY!() {
                continue;
            }

            //if entire memory descriptor is in "low memory"
            if (*descriptor).physical_start + (*descriptor).number_of_pages * 0x1000 < 0x1000000 {
                continue;
            }

            //set physical_start and number of pages, efi memory descriptor could cross low memory boundry
            let mut address:*mut u64 = (*descriptor).physical_start as *mut u64;
            let mut pages:u64 = (*descriptor).number_of_pages;
            if (*descriptor).physical_start < 0x1000000 {
                address = 0x1000000 as *mut u64;
                pages = (*descriptor).number_of_pages - ((0x1000000 - (*descriptor).physical_start) / 0x1000);
            }

            //iterate through pages in the descriptor
            for _ in 0..pages {
                //initialise first l4, l3, l2 and l1
                if l4_start == core::ptr::null_mut() {
                    l4_start = address;
                }
                else if l3_start == core::ptr::null_mut() {
                    l3_start = address;
                }
                else if l2_start == core::ptr::null_mut() {
                    l2_start = address;
                }
                else if l1_start == core::ptr::null_mut() {
                    l1_start = address;
                }

                //if there is no more room to map pages
                else if l4_i == 512 {
                    return;
                }

                //create new paging tables if needed
                //get new address for page table
                //reset index to 0
                //add address in parent table
                //iterate parent table index
                else if l3_i == 512 {
                    l3_start = address;
                    l3_i = 0;
                    *(l4_start.offset(l4_i as isize)) = l3_start as u64;
                    l4_i += 1;
                }
                else if l2_i == 512 {
                    l2_start = address;
                    l2_i = 0;
                    *(l3_start.offset(l3_i as isize)) = l2_start as u64;
                    l2_i += 1;
                }
                else if l1_i == 512 {
                    l1_start = address;
                    l1_i = 0;
                    *(l2_start.offset(l2_i as isize)) = l1_start as u64;
                    l2_i += 1;
                }

                //if none of the above we can assign a new virtual address
                else {
                    *(l1_start.offset(l1_i as isize)) = address as u64;
                    l1_i += 1; 
                }

                address = address.offset(512); //1 page is 512 u64's
            }
        }
    }

    //enable paging
    asm::write_cr3(l4_start as u64);
    asm::write_cr0(cr0);

    //return to long mode
    gdt::exit_compatibility_mode();

    //set cr4.pcide if it was set before
    asm::write_cr4(cr4);
}
