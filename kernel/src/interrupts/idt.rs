//! # Collection of structs and functions to make and load the IDT, as well as functions to prescribe 
//! an interrupt with a handler function. Those function definiotns are also provided
//! 
//! [`Gate`]
//! 
//! [`GateOptions`]
//! 
//! [`IDT`]
//! 
//! [`ExceptionStackFrame`]
//! 
//! 

use core::fmt;
use core::{marker::PhantomData};

/// # Gate
/// 
/// An interrupt gate is the data structure that defines the interrupts within the IDT
/// 
/// Automagically instantiated in the IDT type's new function, thus making a gate struct is never 
/// necessary and only the methods should be used.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Gate<F> {
    offset_low: u16,
    segment_selector: u16,
    pub options: GateOptions,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
    callback: PhantomData<F>,
}

impl<F> Gate<F> {

    // Creates a blank frame, since the pressent bit is not set the CPU does not beleive the interrupt has a handler
    fn empty() -> Self{
        Gate {
            offset_low: 0,
            segment_selector: 0,
            options: GateOptions::new_interrupt_options(),
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
            callback: PhantomData,
        }
    }

    /// # Initialise Gate
    /// 
    /// Initiallises the gate setting the handler address and informing the CPU this interrupt can now be used
    /// 
    /// ## Arguments
    /// * 'callback_addr' - The function as a u64 **Note: This should be achieved by casting the function direcctly to a u64**
    /// eg: `page_fault_handler as u64`
    /// * 'options' - The gate options, the GateOptions struct contains useful constructors for this.
    /// 
    /// ## Returns
    /// * '&mut Self' - This object as a mutable reference. One may call another Gate method after this one to reduce code re-use
    /// 
    /// ## Examples
    /// 
    /// ```
    /// 
    /// let idt: IDT = IDT.new();
    /// idt.breakpoint.init(breakpoint_handler as u64, GateOptions::new_interrupt_options());
    /// ...
    /// extern "x86-interrupt" fn breakpoint_handler(stack_frame: ExceptionStackFrame){
    ///     loop{}
    /// }
    /// ```
    pub fn init(&mut self, callback_addr: u64, options: GateOptions) -> &mut Self{
        self.options = options;
        self.set_handler_address(callback_addr);
        self.segment_selector = 0x8;
        self
    }

    /// # Set address of exception handler
    /// 
    /// Sets the address of the exception handler and sets the present bit. Does nothing else to facilitate a working gate.
    /// To setup a gate with a handler please use Gate.init(), only use this to change the function handler of this interrupt.
    /// 
    /// ## Arguments
    /// * 'address' - the address of the function handler. **Refer to gate.init() documentation to properly get address**
    /// 
    /// ## Returns
    /// * '&mut Self' - This object as a mutable reference. One may call another Gate method after this one to reduce code re-use
    /// 
    /// ## Example
    /// assuming idt.breakpoint has already been initialised
    /// ```
    /// idt.breakpoint.set_handler_address(breakpoint_handler as u64);
    /// ```
    pub fn set_handler_address(&mut self, address: u64) -> &mut Self {
        self.offset_low = address as u16;
        self.offset_mid = (address >> 16) as u16;
        self.offset_high = (address >> 32) as u32;

        self.options.set_present();
        self
    }

    /// # Get address of exception handler
    /// 
    /// Gets the address the gate is currently using as a handler address
    /// 
    /// ## Returns
    /// * 'addr' - The address in the gate
    pub fn get_handler_address(&self) -> u64 {
        let mut addr = self.offset_low as u64;
        addr |= (self.offset_mid as u64) << 16;
        addr |= (self.offset_high as u64) << 32;
        return addr
    }
}

/// # GateOptions
/// 
/// A struct for the easy creation and manipulation of the options bitfields in an IDT gate
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct GateOptions{
    ist: u8,
    options:u8,
}


impl GateOptions{

    /// ## Creates a new GateOptions bitfield for an interrupt gate
    pub fn new_interrupt_options() -> GateOptions {
        return GateOptions{ ist: 0, options: 0b00001110}
    }

    /// ## Creates a new GateOptions bitfield for an call gate
    pub fn new_call_options() -> GateOptions {
        return GateOptions{ ist: 0, options: 0b00001100}
    }

    /// ## Creates a new GateOptions bitfield for an trap gate
    pub fn new_trap_options() -> GateOptions {
        return GateOptions{ ist: 0, options: 0b00001111}
    }

    /// # Set present bit
    /// 
    /// Sets the present bit in the options bitfield, telling the CPU that the address in the gate points to a function.
    /// 
    /// # Returns
    /// * 'Self' - Itself for method chaining
    pub fn set_present(&mut self) -> &mut Self {
        self.options |= 1 << 7u8;
        self
    }

    /// # Clear present bit
    /// 
    /// Clears the present bit in the options bitfield, telling the CPU that the address in the gate points to a function.
    /// 
    /// # Returns
    /// * 'Self' - Itself for method chaining
    pub fn clear_present(&mut self) -> &mut Self {
        self.options &= !(1 << 7u8);
        self
    }

    pub unsafe fn set_stack_index(&mut self, index: u8) -> &mut Self{
        self.ist |= (index + 1) & 0b00000111;
        self
    }
}

/// # ExceptionStackFrame
/// 
/// The data structure fed into every interrupt handler when an interrupt occurs.
#[repr(C)]
pub struct ExceptionStackFrame{
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64
}

impl fmt::Debug for ExceptionStackFrame{

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        f.debug_struct("Gate info: ")
            .field("instruction pointer", &format_args!("{:#x}", self.instruction_pointer))
            .field("stack pointer", &format_args!("{:#x}", self.stack_pointer))
            .field("Cpu flags", &format_args!("{:#b}", self.cpu_flags))
            .field("Code segment", &self.code_segment)
            .field("Stack segment", &self.stack_segment)
            .finish()
    }
}

pub type Interrupt = extern "x86-interrupt" fn (stack_frame: &ExceptionStackFrame) -> ();
pub type Fault = extern "x86-interrupt" fn (stack_frame: &ExceptionStackFrame) -> ();
pub type FaultErr = extern "x86-interrupt" fn (stack_frame: &ExceptionStackFrame, error_code: u64) -> ();
pub type Trap = extern "x86-interrupt" fn (stack_frame: &ExceptionStackFrame) -> ();
pub type Abort = extern "x86-interrupt" fn (stack_frame: &ExceptionStackFrame) -> !;
pub type AbortErr = extern "x86-interrupt" fn (stack_frame: &ExceptionStackFrame, error_code: u64) -> !;
pub type InterruptPageFault = extern "x86-interrupt" fn(stack_frame: &mut ExceptionStackFrame, page_error: u64) -> ();


#[repr(C, packed)]
struct IDTDescriptor {
    size: u16,
    offset: u64,
}

/// # The Interrupt Descriptor Table
/// 
/// A table in memory that tells the CPU how to handle interrupts. A large struct containing interrupt descriptors
/// , or gates, and describes what callback each gate should point to. One with an error field or one without.
/// 
/// Each interrupts is different and may require interrupts to be diabled upon entry, check [https://wiki.osdev.org/Exceptions] for more info
#[repr(C, align(0x10))]
pub struct IDT{
    pub divide_by_zero: Gate<Fault>,
    pub debug: Gate<Fault>,
    pub non_maskable_interupt: Gate<Interrupt>,
    pub breakpoint: Gate<Trap>,
    pub overflow: Gate<Trap>,
    pub bound_range_exceeded: Gate<Fault>,
    pub invalid_opcode: Gate<Fault>,
    pub device_not_available: Gate<Fault>,
    pub double_fault: Gate<AbortErr>,
    coproc_segement_overrun: Gate<Fault>,
    pub invalid_tss: Gate<FaultErr>,
    pub segment_not_present: Gate<FaultErr>,
    pub stack_segment_fault: Gate<FaultErr>,
    pub general_protecion_fault: Gate<FaultErr>,
    pub page_fault: Gate<InterruptPageFault>,
    reserved_1: Gate<Fault>,
    pub x87_floating_point_exception: Gate<Fault>,
    pub alignment_check: Gate<FaultErr>,
    pub machine_check: Gate<Abort>,
    pub simd_floating_point_exception: Gate<Fault>,
    pub virtualisation_exception: Gate<Fault>,
    pub control_protection_exception: Gate<FaultErr>,
    reserved_2: [Gate<Fault>; 6],
    pub hypervisor_injection_exception: Gate<Fault>,
    pub vmm_communication_exception: Gate<FaultErr>,
    pub security_exceptoin: Gate<FaultErr>,
    reserved_3: Gate<Fault>,
    //triple_fault: Gate<Fault>,
    //pub fpu_error: Gate<Fault>,
    pub interrupts: [Gate<Interrupt>; 256 - 32]
}

impl IDT{

    /// # New
    /// Initialises an empty IDT
    /// 
    /// ## Returns 
    /// * 'IDT' - An empty IDT that needs its gates configured.
    #[inline]
    pub fn new() -> IDT{
        IDT{
            divide_by_zero: Gate::empty(),
            debug: Gate::empty(),
            non_maskable_interupt: Gate::empty(),
            breakpoint: Gate::empty(),
            overflow: Gate::empty(),
            bound_range_exceeded: Gate::empty(),
            invalid_opcode: Gate::empty(),
            device_not_available: Gate::empty(),
            double_fault: Gate::empty(),
            coproc_segement_overrun: Gate::empty(),
            invalid_tss: Gate::empty(),
            segment_not_present: Gate::empty(),
            stack_segment_fault: Gate::empty(),
            general_protecion_fault: Gate::empty(),
            page_fault: Gate::empty(),
            reserved_1: Gate::empty(),
            x87_floating_point_exception: Gate::empty(),
            alignment_check: Gate::empty(),
            machine_check: Gate::empty(),
            simd_floating_point_exception: Gate::empty(),
            virtualisation_exception: Gate::empty(),
            control_protection_exception: Gate::empty(),
            reserved_2: [Gate::empty(); 6],
            hypervisor_injection_exception: Gate::empty(),
            vmm_communication_exception: Gate::empty(),
            security_exceptoin: Gate::empty(),
            reserved_3: Gate::empty(),
            //triple_fault: Gate::empty(),
            //fpu_error: Gate::empty(),
            interrupts: [Gate::empty(); 256 - 32]
        }
    }

    /// # Load
    /// 
    /// Loads the IDT for the CPU to use
    #[inline]
    pub fn load(&'static self) -> (){
        unsafe {self.unsafe_load()}
    }

    #[inline]
    unsafe fn unsafe_load(&self) -> (){
        let idtr = IDTDescriptor {
            size: 0x0FFF,
            offset: (self as *const IDT) as u64,
        };
        load_idt(&idtr);
    }
}

extern "C" {
    fn load_idt(_idt_descriptor_pointer: *const IDTDescriptor) -> ();
}