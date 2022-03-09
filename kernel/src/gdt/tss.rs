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

fn load_tss_2(){
    unsafe{
        core::arch::asm!("ltr {0:x}", in(reg) 0x30, options(nomem, nostack, preserves_flags))
    }    
}

impl TSS{

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

    pub fn set_pst_addr(&mut self, index: u8, addr: u64) -> &mut Self{
        self.privelege_stack_table[index as usize] = addr;
        self
    }

    pub fn set_ist_addr(&mut self, index: u8, addr: u64) -> &mut Self{
        self.interrupt_stack_table[index as usize] = addr;
        self
    }

    pub fn load(&self){
        load_tss_2();
    }
}
