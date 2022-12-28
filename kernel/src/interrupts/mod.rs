//! # Module containing all interrupts and functions to initialise the IDT

mod idt;

use lazy_static::lazy_static;
use crate::io::{keyboard, PIC, PS2};
use crate::{println};
use idt::{IDT, GateOptions, ExceptionStackFrame};

lazy_static!{
    static ref IDTABLE: IDT = {
        let mut idt = IDT::new();
        // Add handlers to IDT
        idt.breakpoint.init(breakpoint_handler as u64, GateOptions::new_trap_options());
        idt.page_fault.init(page_fault_handler as u64, GateOptions::new_interrupt_options());
        unsafe{
            idt.double_fault.init(double_fault_handler as u64, GateOptions::new_trap_options())
            .options.set_stack_index(0);
        }
        idt.general_protecion_fault.init(general_protection_handler as u64, GateOptions::new_trap_options());

        // Add keyboard interrupt to the free interupt descriptor
        idt.interrupts[1].init(keyboard_interrupts_handler as u64, GateOptions::new_interrupt_options());
        idt
    };
}

extern "C" {
    fn clear_interrupts() -> ();
    fn set_interrupts() -> ();
}

pub fn init_idt(){
    unsafe{
        clear_interrupts();   
        IDTABLE.load();
        println!(0x0022FF22; "-- Successfully initialised idt");
        // set_interrupts();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: ExceptionStackFrame) -> (){
    println!(0x00FFFF22;  "\nEXCEPTION: BREAKPOINT");
    println!("{:#?}",stack_frame);
    //loop{}
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: ExceptionStackFrame, page_fault_error_code: u64) -> (){
    println!(0x00FFFF22;  "\nEXCEPTION: PAGE FAULT");
    println!("{:#?}",stack_frame);
    println!("Error code: {:#x}", page_fault_error_code);
    loop{}
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: ExceptionStackFrame, error_code: u64) -> !{
    println!(0x00FFFF22;  "\nEXCEPTION: DOUBLE FAULT");
    println!("{:#?}",stack_frame);
    println!("Error code: {:#x}", error_code);
    loop{}
}

extern "x86-interrupt" fn general_protection_handler(stack_frame: ExceptionStackFrame, error_code: u64) -> !{
    println!(0x00FFFF22;  "\nEXCEPTION: GENERAL PROTECTION EXCEPTION");
    println!("{:#?}",stack_frame);
    println!("Error code: {:#x}", error_code);
    loop{}
}

extern "x86-interrupt" fn keyboard_interrupts_handler(_stack_frame: ExceptionStackFrame) -> () {
    let scancode = PS2.lock().read_data();
    let key_stroke = PS2.lock().keystroke_from_ps2_scancode(scancode);

    keyboard::handle_keyboard_for_typing(key_stroke);
    PIC.lock().end_master();
}