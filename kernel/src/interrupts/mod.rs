//! # Module containing all interrupts and functions to initialise the IDT
//! 
//! 

mod idt;

use lazy_static::{lazy_static};
use crate::{println};
use idt::{IDT, GateOptions, ExceptionStackFrame};

lazy_static!{
    //static ref IDTABLE: Mutex<IDT> = Mutex::new(IDT::new());
    static ref IDTABLE: IDT = {
        let mut idt = IDT::new();
        idt.breakpoint.init(breakpoint_handler as u64, GateOptions::new_interrupt_options());
        idt
    };
}

pub fn init_idt(){
    IDTABLE.load();
    println!(0x0022FF22; "-- Successfully initialised idt");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: ExceptionStackFrame){
    println!(0x00FFFF22;  "\nEXCEPTION: BREAKPOINT");
    println!("{:#?}",stack_frame);
    loop{}
}