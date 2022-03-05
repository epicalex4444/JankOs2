use core::arch::asm;

#[inline(always)]
pub fn read_cr0() -> u64 {
    let mut x: u64;
    unsafe {
        asm!("mov {0:r}, cr0", out(reg) x);
    }
    return x;
}

#[inline(always)]
pub fn read_cr2() -> u64 {
    let mut x: u64;
    unsafe {
        asm!("mov {0:r}, cr2", out(reg) x);
    }
    return x;
}

#[inline(always)]
pub fn read_cr3() -> u64 {
    let mut x: u64;
    unsafe {
        asm!("mov {0:r}, cr3", out(reg) x);
    }
    return x;
}

#[inline(always)]
pub fn read_cr4() -> u64 {
    let mut x: u64;
    unsafe {
        asm!("mov {0:r}, cr4", out(reg) x);
    }
    return x;
}

#[inline(always)]
pub fn read_cr8() -> u64 {
    let mut x: u64;
    unsafe {
        asm!("mov {0:r}, cr8", out(reg) x);
    }
    return x;
}

#[inline(always)]
pub fn write_cr0(x: u64) -> () {
    unsafe {
        asm!("mov cr0, {0:r}", in(reg) x);
    }
}

#[inline(always)]
pub fn write_cr2(x: u64) -> () {
    unsafe {
        asm!("mov cr2, {0:r}", in(reg) x);
    }
}

#[inline(always)]
pub fn write_cr3(x: u64) -> () {
    unsafe {
        asm!("mov cr3, {0:r}", in(reg) x);
    }
}

#[inline(always)]
pub fn write_cr4(x: u64) -> () {
    unsafe {
        asm!("mov cr4, {0:r}", in(reg) x);
    }
}

#[inline(always)]
pub fn write_cr8(x: u64) -> () {
    unsafe {
        asm!("mov cr8, {0:r}", in(reg) x);
    }
}

//read model specific register
#[inline(always)]
pub fn rdmsr(msr: u32) -> u64 {
    let mut low: u32;
    let mut high: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") msr,
            lateout("eax") low,
            lateout("edx") high
        );
    }
    return ((high as u64) << 32) + (low as u64);
}

//write model specific register
#[inline(always)]
pub fn wrmsr(msr: u32, value: u64) -> () {
    let low: u32 = (value & 0xFFFFFFFF) as u32;
    let high: u32 = (value >> 32) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("ecx") msr,
            in("eax") low,
            in("edx") high
        );
    }
}

#[inline(always)]
pub fn read_efer() -> u64 {
    return rdmsr(0xC0000080u32);
}

#[inline(always)]
pub fn write_efer(value: u64) -> () {
    wrmsr(0xC0000080u32, value);
}

//sends byte to port
#[inline(always)]
pub fn outb(port: u16, value: u8) -> () {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}

//sends word to port
#[inline(always)]
pub fn outw(port: u16, value: u16) -> () {
    unsafe {
        asm!("out dx, ax", in("dx") port, in("ax") value);
    }
}

//sends long to port
#[inline(always)]
pub fn outl(port: u16, value: u32) -> () {
    unsafe {
        asm!("out dx, eax", in("eax") value, in("dx") port,);
    }
}

//gets byte from port
#[inline(always)]
pub fn inb(port: u16) -> u8 {
    let x: u8;
    unsafe {
        asm!("in al, dx", out("al") x, in("dx") port);
    }
    return x;
}

//gets word from port
#[inline(always)]
pub fn inw(port: u16) -> u16 {
    let x: u16;
    unsafe {
        asm!("in ax, dx", out("ax") x, in("dx") port);
    }
    return x;
}

//gets long from port
#[inline(always)]
pub fn inl(port: u16) -> u32 {
    let x: u32;
    unsafe {
        asm!("in eax, dx", in("dx") port, out("eax") x);
    }
    return x;
}

//no operations
#[inline(always)]
pub fn nop() -> () {
    unsafe {
        asm!("nop");
    }
}

//cpu halt stops cpu execution
#[inline(always)]
pub fn hlt() -> () {
    unsafe {
        asm!("hlt");
    }
}

//eax is used to input into cpuid and it also ouput to
//in some cases registers can contain undefined values
#[inline(always)]
pub fn cpuid(eax: *mut u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32) -> () {
    unsafe {
        asm!(
            "cpuid",
            "mov {:e}, ebx",
            out(reg) *ebx,
            inlateout("eax") *eax,
            lateout("ecx") *ecx,
            lateout("edx") *edx
        );
    }
}

//clears interupts
#[inline(always)]
pub fn cli() -> () {
    unsafe {
        asm!("cli");
    }
}