mod tss;

use crate::{println};
use core::{mem::size_of, ops::DerefMut};
use spin::Mutex;
use tss::TSS;

#[repr(C, packed)]
struct GDTDescriptor {
    size: u16,
    offset: u64,
}

struct Segment {
    segment_descriptor: u64,
}

impl Segment {
    #[inline]
    const fn new(access: u8, flags: u8, limit: u32) -> Segment {
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
    pub const fn new(access: u8, flags: u8, limit: u32, base: u64) -> SysSegment {
        // Push access byte into bites 47-40 of the first 64-bit segment
        let mut seg_1 = (access as u64) << 40;

        // Push flags into bits 55-52
        seg_1 |= ((flags & 0b00001111) as u64) << 52;

        // Push first 16 limit bits 
        seg_1 |= ((limit & 0xFFFF) as u64) << 0;
        // Push last 4 limit bits to 51 - 48
        seg_1 |= (((limit & 0xF0000) >> 16) as u64) << 48;

        seg_1 |= (base & 0xFFFF) << 16;
        seg_1 |= ((base & 0xFF0000) >> 16) << 32;
        seg_1 |= ((base & 0xFF000000) >> 20) << 56;

        let seg_2 = base & 0xFFFFFFFF00000000;

        SysSegment {
            segment_first: seg_1,
            segment_second: seg_2,
        }
    }

    pub fn set_tss(&mut self, tss: &TSS) -> () {
        self.set_base((tss as *const TSS) as u64);
        self.set_limit(size_of::<TSS>() as u64);
    }

    fn set_base(&mut self, addr: u64) -> () {
        self.segment_first |= (addr & 0xFFFF) << 16;
        self.segment_first |= ((addr & 0xFF0000) >> 16) << 32;
        self.segment_first |= ((addr & 0xFF000000) >> 20) << 56;

        self.segment_second = (addr & 0xFFFFFFFF00000000) >> 32;
    }

    fn set_limit(&mut self, limit: u64) -> () {
        self.segment_first |= ((limit & 0xFFFF) as u64) << 0;
        self.segment_first |= (((limit & 0xFFFF0000) >> 16) as u64) << 48;
    }
}

#[repr(C, packed)]
pub struct GDTable {
    null: Segment,
    kernel_code: Segment,
    kernel_data: Segment,
    // user_null: Segment,
    user_code: Segment,
    user_data: Segment,
    task_state: SysSegment,
}

impl GDTable {
    pub const fn new() -> GDTable {
        GDTable {
            null: Segment::new(0, 0, 0),
            kernel_code: Segment::new(0x9A, 0xA, 0xFFFFF),
            kernel_data: Segment::new(0x92, 0xA, 0xFFFFF),
            // user_null: Segment::new(0, 0, 0),
            user_code: Segment::new(0x9A, 0xA, 0xFFFFF),
            user_data: Segment::new(0x92, 0xA, 0xFFFFF),
            task_state: SysSegment::new(0x89, 0x4, 0, 0),
        }
    }

    // Spawning new descriptors should not matter as only one will ever be in scope and the rest will be marked
    // for override.
    fn get_descriptor(&self) -> GDTDescriptor {
        GDTDescriptor {
            size: (size_of::<GDTable>() as u16) - 1,
            offset: (self as *const GDTable) as u64,
        }
    }

    pub fn load(&self) -> &Self {
        unsafe {
            load_gdt(&self.get_descriptor());
            println!(0x0022FF22; "-- Loaded GDT");
            reload_segments();
            println!(0x0022FF22; "-- Reloaded CS registers");
        }
        self
    }
}

extern "C" {
    fn load_gdt(_gdt_descriptor_pointer: *const GDTDescriptor) -> ();
    fn reload_segments() -> ();
}

pub static TABLE: Mutex<GDTable> = Mutex::new(GDTable::new());
static TSS: Mutex<TSS> = Mutex::new(TSS::new());

// ------- THE TEMPORARY ZONE
const STACK_SIZE: usize = 4096 * 5;
static DUMMY_STACK:Mutex<u64> = Mutex::new(0);

fn init_dummy_stack(){
    static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

    let stack_start = unsafe { &STACK as *const [u8; STACK_SIZE]} as u64;
    println!(0x00F55F22; "IST1 Stack start: {:#x}", stack_start);
    let stack_end = stack_start + (STACK_SIZE as u64);
    println!(0x00F55F22; "IST1 Stack end: {:#x}", stack_end);
    *DUMMY_STACK.lock().deref_mut() = stack_end;
}
// -------- END TEMPORARY ZONE

pub fn init_gdt() -> () {
    let mut table = TABLE.lock();
    let mut tss = TSS.lock();

    table.task_state.set_tss(&*tss);

    init_dummy_stack();
    tss.interrupt_stack_table[0] = *DUMMY_STACK.lock();

    table.load();
    tss.load();  
}