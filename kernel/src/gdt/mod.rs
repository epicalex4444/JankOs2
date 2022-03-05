use crate::println;
use core::mem::size_of;
use lazy_static::{lazy_static, __Deref};

#[repr(C, packed)]
struct GDTDescriptor {
    size: u16,
    offset: u64,
}

struct Segment {
    segment_descriptor: u64,
}

impl Segment {
    pub fn new(access: u8, flags: u8, limit: u32) -> Segment {
        let mut seg = (access as u64) << 40;
        seg |= ((flags & 0b00001111) as u64) << 52;

        seg |= ((limit & 0xFFFF) as u64) << 0;
        seg |= (((limit & 0xF0000) >> 16) as u64) << 48;
        return Segment {
            segment_descriptor: seg,
        };
    }
}

#[repr(C, packed)]
struct SysSegment {
    segment_first: u64,
    segment_second: u64,
}

impl SysSegment {
    pub fn new(access: u8, flags: u8, limit: u32, base: u64) -> SysSegment {
        let mut seg_1 = (access as u64) << 40;

        seg_1 |= ((flags & 0b00001111) as u64) << 52;

        seg_1 |= ((limit & 0xFFFF) as u64) << 0;
        seg_1 |= (((limit & 0xF0000) >> 16) as u64) << 48;

        seg_1 |= (base & 0xFFFF) << 16;
        seg_1 |= ((base & 0xFF0000) >> 16) << 32;
        seg_1 |= ((base & 0xFF000000) >> 20) << 56;

        let seg_2 = base & 0xFFFFFFFF00000000;

        let mut sys_seg = SysSegment {
            segment_first: seg_1,
            segment_second: seg_2,
        };
        sys_seg.set_base();
        sys_seg.set_limit();
        return sys_seg;
    }

    fn set_base(&mut self) -> () {
        self.segment_first |= ((self as *mut SysSegment) as u64 & 0xFFFF) << 16;
        self.segment_first |= (((self as *mut SysSegment) as u64 & 0xFF0000) >> 16) << 32;
        self.segment_first |= (((self as *mut SysSegment) as u64 & 0xFF000000) >> 20) << 56;

        self.segment_second = ((self as *mut SysSegment) as u64 & 0xFFFFFFFF00000000) >> 32;
    }

    fn set_limit(&mut self) -> () {
        let limit = size_of::<SysSegment>();
        self.segment_first |= ((limit & 0xFFFF) as u64) << 0;
        self.segment_first |= (((limit & 0xFFFF0000) >> 16) as u64) << 48;
    }
}

#[repr(C, packed)]
struct GDTable {
    null: Segment,
    kernel_code: Segment,
    kernel_data: Segment,
    user_null: Segment,
    user_code: Segment,
    user_data: Segment,
    //task_state: SysSegment,
}
 
lazy_static! {
    static ref TABLE: GDTable = GDTable {
        null: Segment::new(0, 0, 0),
        kernel_code: Segment::new(0x9A, 0xA, 0xFFFFF),
        kernel_data: Segment::new(0x92, 0xA, 0xFFFFF),
        user_null: Segment::new(0,0,0),
        user_code: Segment::new(0x9A, 0xA, 0xFFFFF),
        user_data: Segment::new(0x92, 0xA, 0xFFFFF),
        //task_state: SysSegment::new(0x89, 0x0, 0, 0),
    };
}

extern "C" {
    fn load_gdt(_gdt_descriptor_pointer: *const GDTDescriptor) -> ();
    fn reload_segments() -> ();
}

pub fn init_gdt() -> () {
    unsafe {
        let gdt_desc = GDTDescriptor {
            size: (size_of::<GDTable>() as u16) - 1,
            offset: (TABLE.deref() as *const GDTable) as u64,
        };

        load_gdt(&gdt_desc);
        println!(0x0022FF22; "-- Loaded GDT");
        reload_segments();
        println!(0x0022FF22; "-- Reloaded CS registers");
    }
}