use core::mem::size_of;

#[repr(packed)]
struct GDT {
    pub null:u64,
    pub kernel_code:u64,
    pub kernel_data:u64,
}

#[repr(C, packed)]
struct GDTDescriptor {
    pub size:u16,
    pub offset:u64
}

//returns a gdt_segment
//flags is actually a u4, limit is actually a u24
//
//access
//bit 7: must be 1
//bit 6-5: privilege 0-3, 0 is highest
//bit 4: 0 is a system segment, 1 is a code or data segment
//bit 3: 0 is a data segment, 1 means is a code segment
//bit 2: data segment 0 grows up, 1 grows down
//       code segment 0 it can't be executed from lower priveleges, 1 it can
//bit 2: data segments 0 reading is not allowed, 1 it is
//       code segments 0 writing is not allowed, 1 it is
//bit 1: must be 0
//
//flags
//bit 3: 0 limit is byte block, 1 limit is a page block
//bit 2: 0 is 16 bit mode, 1 is 32 bit mode
//       0 for 64 bit code segments, 1 for 64 bit data segments
//bit 1: 1 for 64 bit code segments else 0
//bit 0: must be 0
const fn segment(access: u8, flags: u8, limit: u32) -> u64 {
    let mut seg = (access as u64) << 40;
    seg |= ((flags & 0b00001111) as u64) << 52;
    seg |= ((limit & 0xFFFF) as u64) << 0;
    seg |= (((limit & 0xF0000) >> 16) as u64) << 48;
    return seg;
}

static GDT_64:GDT = GDT {
    null: 0,
    kernel_code: segment(0b10011010, 0b1010, 0xFFFFF),
    kernel_data: segment(0b10010010, 0b1100, 0xFFFFF),
};

static GDT_32:GDT = GDT {
    null: 0,
    kernel_code: segment(0b10011010, 0b1100, 0xFFFFF),
    kernel_data: segment(0b10010010, 0b1100, 0xFFFFF),
};

//offset is set to 0 since it can't be known at compile time
static mut GDT_DESCRIPTOR_64:GDTDescriptor = GDTDescriptor {
    size: size_of::<GDT>() as u16 - 1,
    offset: 0
};

//offset is set to 0 since it can't be known at compile time
static mut GDT_DESCRIPTOR_32:GDTDescriptor = GDTDescriptor {
    size: size_of::<GDT>() as u16 - 1,
    offset: 0
};

//externel reference to load_gdt function in load_gdt.asm
extern {
    fn load_gdt(_gdt_descriptor_ptr:*const GDTDescriptor) -> ();
    fn reload_segments_32() -> ();
}

//initialises the 64 bit and 32 bit gdt's then loads the 64bit gdt
pub fn init_gdt() -> () {
    unsafe {
        GDT_DESCRIPTOR_64.offset = (&GDT_64 as *const GDT) as u64;
        GDT_DESCRIPTOR_32.offset = (&GDT_32 as *const GDT) as u64;
        load_gdt(&GDT_DESCRIPTOR_64);
    }
}

//loads the 32 bit gdt
pub fn enter_compatibility_mode() -> () {
    unsafe {
        load_gdt(&GDT_DESCRIPTOR_32);
    }
}

//loads the 64 bit gdt
pub fn exit_compatibility_mode() -> () {
    unsafe {
        load_gdt(&GDT_DESCRIPTOR_64);
    }
}