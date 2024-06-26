#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(multi_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]


#[allow(dead_code)]

mod interrupts;
mod serial;
mod vga;
mod gdt;
use::core::arch::asm;
#[no_mangle]

pub extern "C" fn _start() {
    println!("Hello World{}", "!");
    multi_os::init();
    
    
    #[cfg(test)]
     test_main();

    println!("It did not crash!");
     
     
    multi_os::hlt_loop();
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

