use super::{in_b, out_b};

#[derive(Clone, Copy)]
pub enum PicOrder {
    Master = 0x4,
    Slave = 0x2,
}


const ICW1_INIT: u8 = 0x0010;
const ICW1_ICW4: u8 = 0x0001;
const ICW4_8086: u8 = 0x0001;
const PIC_EOI: u8 = 0x0020;

pub const PIC_MASTER_OFFSET: u8 = 32;
pub const PIC_SLAVE_OFFSET: u8 = PIC_MASTER_OFFSET + 8;

pub struct PIC {
    address: u16,
    order: PicOrder,
}

impl PIC {
    pub const fn new(addr: u16, ord: PicOrder) -> PIC {
        PIC {
            address: addr,
            order: ord,
        }
    }

    fn in_command(&self) -> u8 {
        unsafe { in_b(self.address) }
    }

    fn in_data(&self) -> u8 {
        unsafe { in_b(self.address + 1) }
    }

    fn out_command(&self, val: u8) -> () {
        unsafe {
            out_b(self.address, val);
        }
    }

    fn out_data(&self, val: u8) -> () {
        unsafe {
            out_b(self.address + 1, val);
        }
    }

    pub fn end(&self) -> () {
        self.out_command(PIC_EOI);
    }

    pub fn remap(&self, offset: u8) -> &Self {
        let bitmask = self.in_data();
        self.out_command(ICW1_INIT | ICW1_ICW4);
        self.out_data(offset);
        self.out_data(self.order as u8);
        self.out_data(ICW4_8086);
        self.out_data(bitmask);
        self
    }
}

pub struct ChainedPIC {
    master: PIC,
    slave: PIC,
}

impl ChainedPIC {
    pub const fn new() -> ChainedPIC {
        ChainedPIC {
            master: PIC::new(0x20, PicOrder::Master),
            slave: PIC::new(0xA0, PicOrder::Slave),
        }
    }

    pub fn remap(&self) -> &Self {
        self.master.remap(PIC_MASTER_OFFSET);
        self.slave.remap(PIC_SLAVE_OFFSET);
        self
    }

    pub fn set_interrupt_mask(&self, mask_master: u8, mask_slave: u8) -> &Self {
        self.master.out_data(mask_master);
        self.slave.out_data(mask_slave);
        self
    }

    pub fn end_master(&self) -> &Self {
        self.master.end();
        self
    }

    pub fn end_slave(&self) -> &Self {
        self.slave.end();
        self.master.end();
        self
    }
}

/*

pub fn remap_pic() -> () {
    unsafe{
        let bitmask_1: u8;
        let bitmask_2: u8;
        bitmask_1 = in_b(MASTER_DATA);
        wait();
        bitmask_2 = in_b(SLAVE_DATA);
        wait();

        out_b(MASTER_COMMAND, ICW1_INIT | ICW1_ICW4);
        wait();
        out_b(SLAVE_COMMAND, ICW1_INIT | ICW1_ICW4);
        wait();

        out_b(MASTER_DATA, PIC_MASTER_OFFSET);
        wait();
        out_b(SLAVE_DATA, PIC_SLAVE_OFFSET);
        wait();

        out_b(MASTER_DATA, 0x0004);
        wait();
        out_b(SLAVE_DATA, 2);
        wait();

        out_b(MASTER_DATA,ICW4_8086);
        wait();
        out_b(SLAVE_DATA, ICW4_8086);
        wait();

        out_b(MASTER_DATA, bitmask_1);
        wait();
        out_b(SLAVE_DATA, bitmask_2);
        wait();
    }

}

pub fn unmask_keyboard() -> () {
    unsafe{
        out_b(MASTER_DATA, 0b11111101);
        out_b(SLAVE_DATA, 0b11111111);
    }
}

pub fn end_master() -> () {
    unsafe{
        out_b(MASTER_COMMAND, PIC_EOI);
    }

}

pub fn end_slave() -> () {
    unsafe{
        out_b(SLAVE_COMMAND, PIC_EOI);
        out_b(MASTER_COMMAND, PIC_EOI);
    }
}
*/
