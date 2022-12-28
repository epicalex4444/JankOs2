//!
//! Types for describing the Global Descriptor Table as well as functions for loading it.
//! https://wiki.osdev.org/Global_descriptor_table

mod tss;

use crate::{println};
use core::{mem::size_of, ops::DerefMut};
use spin::Mutex;
use tss::TSS;

/// Describes the loacation of the [`GDTable`]. This will be provided by the bootloader so this acts only as a 
/// representation and should not be instantiated.
#[repr(C, packed)]
struct GDTDescriptor {
    size: u16,
    offset: u64,
}

/// The segemnt descriptor, Describes each segment in the [`GDTable`].\ 
/// Details at https://wiki.osdev.org/Global_descriptor_table#Segment_Descriptor.
/// 
/// The basic table:
/// 
/// | 63   | 56  |\|| 55  | 52   |\|| 51  | 48   |\|| 47   | 40        |\|| 39  | 32  |
/// | :--  | --: |--| :-- | --:  |--| :-- | --:  |--| :--  | --:       |--| :-- | --: | 
/// | **Base**  ||\|| **Flags** ||\|| **Limit** ||\|| **Access Byte** ||\|| **Base** ||
/// | 31   | 24  |\|| 3   | 0    |\|| 19  | 16   |\|| 7    | 0         |\|| 23  | 16  |
/// |**31**|     |  |     |      |  |     |**16**|\||**15**|           |  |     |**0**|
/// | **Base**  ||  |     |      |  |     |      |\|| **Limit**       ||  |     |     |
/// | 15   |     |  |     |      |  |     | 0    |\|| 15   |           |  |     | 0   |
struct Segment {
    segment_descriptor: u64,
}

impl Segment {
    /// Creates a new segemnt with the given access flags and limit.
    /// 
    /// # Arguments
    /// * `access`: The access byte.
    /// * `flags`: The segment's flags, only the first 4 bits are used.
    /// * `limit`: The maximum addressable unit of the segment in pages. Unused and ignored in long mode.
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

/// Describes a System Segment for long mode, a special type of segement that holds the [`TSS`].\
/// Details: https://wiki.osdev.org/Global_descriptor_table#Long_Mode_System_Segment_Descriptor
#[repr(C, packed)]
struct SysSegment {
    segment_first: u64,
    segment_second: u64,
}

impl SysSegment {

    /// Creates a new [`SysSegment`]
    /// 
    /// # Arguments
    /// * `access`: The System segments access byte, the first 4 bits are different to the regular [`Segment`].
    /// * `flags`: The segment's flags, only the first 4 bits are used.
    /// * `limit`: The maximum addressable unit of the segment in bytes.
    /// * `base`: The address of the [`TSS`] or [`LDT`] it describes.
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

    /// Sets the address and limit of the given [`TSS`] reference in this segment. 
    pub fn set_tss(&mut self, tss: &TSS) -> () {
        self.set_base((tss as *const TSS) as u64);
        self.set_limit(size_of::<TSS>() as u64);
    }

    // Sets the bits of the address into the correct sections of the segment.
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

/// The global descriptor table type. Describes and points to secure parts of memory i short mode. In long mode, 
/// paging reservation is the forced memory safety but this is still required and the pointers are useful.\
/// This struct explicitly defines this.\
/// https://wiki.osdev.org/Global_descriptor_table#Long_Mode_System_Segment_Descriptor
#[repr(C, packed)]
pub struct GDTable {
    null: Segment,
    kernel_code: Segment,
    kernel_data: Segment,
    user_code: Segment,
    user_data: Segment,
    task_state: SysSegment,
}

impl GDTable {
    /// Creates a new, ready for use, [`GDTable`]. The result should not be modified.
    pub const fn new() -> GDTable {
        GDTable {
            null: Segment::new(0, 0, 0),
            kernel_code: Segment::new(0x9A, 0xA, 0xFFFFF),
            kernel_data: Segment::new(0x92, 0xA, 0xFFFFF),
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

/// The global GDT instance.
pub static TABLE: Mutex<GDTable> = Mutex::new(GDTable::new());
static TSS: Mutex<TSS> = Mutex::new(TSS::new());

// ------- THE TEMPORARY ZONE
const STACK_SIZE: usize = 4096 * 5;
static DOUBLE_FAULT_STACK:Mutex<u64> = Mutex::new(0);

fn init_double_fault_stack(){
    static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

    let stack_start = unsafe { &STACK as *const [u8; STACK_SIZE]} as u64;
    println!(0x00F55F22; "IST1 Stack start: {:#x}", stack_start);
    let stack_end = stack_start + (STACK_SIZE as u64);
    println!(0x00F55F22; "IST1 Stack end: {:#x}", stack_end);
    *DOUBLE_FAULT_STACK.lock().deref_mut() = stack_end;
}

/// Sets up the GDT for ring 0 operation, including setting cpu addresses. 
pub fn init_gdt() -> () {
    let mut table = TABLE.lock();
    let mut tss = TSS.lock();

    table.task_state.set_tss(&*tss);

    init_double_fault_stack();
    tss.interrupt_stack_table[0] = *DOUBLE_FAULT_STACK.lock();

    table.load();
    tss.load();  
}
// -------- END TEMPORARY ZONE