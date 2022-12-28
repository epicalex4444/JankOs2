//!
//! Long mode Task State Segment types and functions
//! https://wiki.osdev.org/Task_State_Segment#Long_Mode

/// The Task state segemnt struct designed for long mode operation. It stores the privelege stack table and the 
/// interrupt stack table.
/// 
/// - The privelege stack table holds the pointers to stacks for when privelege level changes from low to high.
/// - Interrupt stack table holds pointers to stacks used by the [`IDT`]
#[repr(C, packed)]
pub struct TSS{
    reserved_1: u32,
    pub privelege_stack_table: [u64; 3],
    reserved_2: u64,
    pub interrupt_stack_table: [u64; 7],
    reserved_3: u64,
    reserved_4: u16,
    pub iodp: u16,
}

extern "C"{
    fn load_tss() -> ();
}

impl TSS{

    /// Creates a completely blank TSS.
    pub const fn new() -> TSS{
        TSS{
            reserved_1: 0,
            reserved_2: 0,
            reserved_3: 0,
            reserved_4: 0,
            privelege_stack_table: [0; 3],
            interrupt_stack_table: [0; 7],
            iodp: 0,
        }
    }

    /// Sets the address of privelege stack pointer at [`index`].
    pub fn set_pst_addr(&mut self, index: u8, addr: u64) -> &mut Self{
        self.privelege_stack_table[index as usize] = addr;
        self
    }

    /// Sets the address of interrupt stack pointer at [`index`].
    pub fn set_ist_addr(&mut self, index: u8, addr: u64) -> &mut Self{
        self.interrupt_stack_table[index as usize] = addr;
        self
    }

    /// Loads the TSS into the proper register.
    pub fn load(&self){
        unsafe {load_tss();}
    }
}
