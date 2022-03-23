//! # Module containing all interrupts and functions to initialise the IDT
//! 
//! 

mod idt;

use lazy_static::lazy_static;
use crate::io::{keyboard::KeyStroke, PIC, PS2};
use crate::{println, print};
use idt::{IDT, GateOptions, ExceptionStackFrame};

lazy_static!{
    static ref IDTABLE: IDT = {
        let mut idt = IDT::new();
        idt.breakpoint.init(breakpoint_handler as u64, GateOptions::new_trap_options());
        idt.page_fault.init(page_fault_handler as u64, GateOptions::new_interrupt_options());
        unsafe{
            idt.double_fault.init(double_fault_handler as u64, GateOptions::new_trap_options())
            .options.set_stack_index(0);
        }
        idt.general_protecion_fault.init(general_protection_handler as u64, GateOptions::new_trap_options());
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
        //set_interrupts();
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


    match key_stroke.stroke {
        KeyStroke::Pressed => {
            
        },
        KeyStroke::Released => {

            if let Some(character) = key_stroke.code.character_key_to_char(){
                print!("{}",character);
            }
            else if let Some(num) = key_stroke.code.number_key_to_int(){
                print!("{}", num);
            }
            
        },
        KeyStroke::Unknown => {}
    }
    PIC.lock().end_master();
}