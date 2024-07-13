
use indexed_valued_enums::indexed_enum::Indexed;
use indexed_valued_enums::valued_enum::Valued;
use pic8259::ChainedPics; 
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};
use crate::println;
use crate::gdt;

use lazy_static::lazy_static;
pub const PIC_1_OFFSET:u8=32;
pub const PIC_2_OFFSET:u8=PIC_1_OFFSET+8;
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });
  
    pub const  byte: u8 = 32;
  pub const y: usize = byte as usize;
   
 
use indexed_valued_enums::{enum_valued_as,Valued};
  use crate::print;
  
#[derive(PartialEq, Debug, Valued)]
#[enum_valued_as(u8)]  

   pub enum InterruptIndex {
       #[value(32)]
        Timer,
        #[value(33)]
        Keyboard,
    }
    
  
  extern "x86-interrupt" fn timer_interrupt_handler(
      _stack_frame: InterruptStackFrame)
  {
      print!(".");
      unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.value());
    }
  } 
    


  
    lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(bk_handler);
        unsafe {
          idt.double_fault.set_handler_fn(double_fault_handler)
              .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
      }
        idt.divide_error.set_handler_fn(divide_by_zero_handler);
        idt[InterruptIndex::Timer.discriminant()+y].set_handler_fn(timer_interrupt_handler); 
        idt[InterruptIndex::Keyboard.discriminant()+y].set_handler_fn(keyboard_interrupt_handler);
        idt.non_maskable_interrupt.set_handler_fn(nmi_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.page_fault.set_handler_fn(page_fault_handler); 
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.virtualization.set_handler_fn(virtualization_check);
        idt.security_exception.set_handler_fn(security_exception_handler);

        idt
    };
}
extern "x86-interrupt" fn keyboard_interrupt_handler(
  _stack_frame: InterruptStackFrame)
{ use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(ScancodeSet1::new(),
            layouts::Us104Key, HandleControl::Ignore)
        );
}

let mut keyboard = KEYBOARD.lock();
let mut port = Port::new(0x60);

let scancode: u8 = unsafe { port.read() };
if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
    if let Some(key) = keyboard.process_keyevent(key_event) {
        match key {
            DecodedKey::Unicode(character) => print!("{}", character),
            DecodedKey::RawKey(key) => print!("{:?}", key),
        }
    }
}

  unsafe {
      PICS.lock()
          .notify_end_of_interrupt(InterruptIndex::Keyboard.value());
  }
}
pub fn init_idt() {
    IDT.load();
}

  extern "x86-interrupt" fn bk_handler(stack_frame:InterruptStackFrame){
    println!("exception:breakpoint\n{:#?}",stack_frame);
  }
  extern "x86-interrupt" fn divide_by_zero_handler(_stack_frame: InterruptStackFrame) {
    // In a real application, you might log this error or halt the system
    println!("EXCEPTION: DIVIDE BY ZERO");
}
extern "x86-interrupt" fn double_fault_handler(
  stack_frame: InterruptStackFrame, _error_code: u64) -> !
{
  panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn nmi_handler(stack_frame: InterruptStackFrame){
  panic!("NMI \n{:#?}", stack_frame);
}
extern "x86-interrupt" fn bound_handler(stack_frame: InterruptStackFrame){
  panic!("bound_range exceeded \n{:#?}", stack_frame);
}
extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame){
  panic!("invalid_opcode \n{:#?}", stack_frame);
}

extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame){
  panic!("device_not_available_handler\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(stack_frame: InterruptStackFrame, _error_code: u64) {
  panic!("invalid_tss_handler \n{:#?}", stack_frame);
}

extern "x86-interrupt" fn segment_not_present_handler(stack_frame: InterruptStackFrame, _error_code: u64){
  panic!(" segment_not_present_handler\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64){
  panic!("stack_segment_fault \n{:#?}", stack_frame);
}

extern "x86-interrupt" fn general_protection_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64){
  panic!("general_protection_fault \n{:#?}", stack_frame);
}



extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: InterruptStackFrame){
  panic!("x87_floating_point \n{:#?}", stack_frame);
}

extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: InterruptStackFrame){
  panic!("simd_floating_point \n{:#?}", stack_frame);
}

extern "x86-interrupt" fn alignment_check_handler(stack_frame: InterruptStackFrame, _error_code: u64){
  panic!("alignment_check \n{:#?}", stack_frame);
}

extern "x86-interrupt" fn virtualization_check(stack_frame: InterruptStackFrame){
  panic!("virtualization_check \n{:#?}", stack_frame);
}

extern "x86-interrupt" fn security_exception_handler(stack_frame: InterruptStackFrame, _error_code: u64){
  panic!("security_exception \n{:#?}", stack_frame);
}
use x86_64::structures::idt::PageFaultErrorCode;
pub fn hlt_loop() -> ! {
  loop {
      x86_64::instructions::hlt();
  }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    hlt_loop();
}