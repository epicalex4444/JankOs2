use crate::asm::cli;
use core::{arch::asm, mem::size_of};
use lazy_static::lazy_static;

#[repr(C, packed)]
struct GDTDescriptor {
    size: u16,
    offset: u64,
}

struct Segment {
    segment_descriptor: u64,
}

#[repr(C, packed)]
struct SysSegment {
    segment_first: u64,
    segment_second: u64,
}

impl SysSegment {
    pub fn new(access: u8, flags: u8, limit: u32, base: u64) -> SysSegment {
        let mut seg_1 = 0u64;
        seg_1 = (access as u64) << 40;

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

        self.segment_second = (self as *mut SysSegment) as u64 & 0xFFFFFFFF00000000;
    }

    fn set_limit(&mut self) -> () {
        let limit = size_of::<SysSegment>();
        self.segment_first |= ((limit & 0xFFFF) as u64) << 0;
        self.segment_first |= (((limit & 0xFFFF0000) >> 16) as u64) << 48;
    }
}

impl Segment {
    pub fn new(access: u8, flags: u8, limit: u32) -> Segment {
        let mut seg = 0u64;
        seg = (access as u64) << 40;
        seg |= ((flags & 0b00001111) as u64) << 52;

        seg |= ((limit & 0xFFFF) as u64) << 0;
        seg |= (((limit & 0xF0000) >> 16) as u64) << 48;
        return Segment {
            segment_descriptor: seg,
        };
    }
}

struct GDTable {
    null: Segment,
    kernel_code: Segment,
    kernel_data: Segment,
    user_code: Segment,
    user_data: Segment,
    task_state: SysSegment,
}

lazy_static! {
    static ref TABLE: GDTable = GDTable {
        null: Segment::new(0, 0, 0),
        kernel_code: Segment::new(0x9A, 0xA, 0xFFFFF),
        kernel_data: Segment::new(0x92, 0xC, 0xFFFFF),
        user_code: Segment::new(0xFA, 0xA, 0xFFFFF),
        user_data: Segment::new(0xF2, 0xC, 0xFFFFF),
        task_state: SysSegment::new(0x89, 0x0, 0, 0),
    };
}

pub fn init_gdt() -> () {
    cli();
    unsafe {
        let gdt_desc = GDTDescriptor {
            size: size_of::<TABLE>() as u16 - 1,
            offset: (&TABLE as *const TABLE) as u64,
        };
        lgdt(&gdt_desc);
    }
}

fn lgdt(gdtr: *const GDTDescriptor) {
    unsafe {
        asm!("lgdt [rdi]");
    }
}
