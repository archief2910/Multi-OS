#![no_std]
#![no_main]
#![feature(maybe_uninit_slice)]
#![feature(custom_test_frameworks)]
#![test_runner(multi_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]
#[allow(dead_code)]
extern crate alloc;


pub mod interrupts;
pub mod comphysical;
mod serial;
mod vga;
mod gdt;
use::core::arch::asm;
use bootloader::{BootInfo, entry_point};

entry_point!(_start);


 fn _start(boot_info: &'static BootInfo)->!{
    use x86_64::VirtAddr;
    
    println!("/"); println!("\\"); println!("/");println!("\\");
   println!("/");         println!("\\/");        println!("\\");
  println!("/");                                   println!("\\");
    multi_os::init();


   

    
    
    
    #[cfg(test)]
     test_main();

    println!("It did not crash!");
     
     
    multi_os::hlt_loop();
}
fn invalid_opcode_exception() {
    unsafe {
        // Trigger a security exception. This is just an example and may need to be adjusted based on your environment.
        asm!("ud2", options(nomem, nostack, preserves_flags));
    }
}
fn trigger_divide_by_zero() {
    unsafe {
        asm!(
            "mov eax, 1", // Move 1 into the EAX register
            "xor edx, edx", // Zero out the EDX register (high part of EAX)
            "div edx", // Divide EAX by EDX (1 / 0), which will cause a divide-by-zero exception
            options(nostack, nomem),
        );
    }
    // Trigger a divide by zero exception
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

