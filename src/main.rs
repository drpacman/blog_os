#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC!!: {}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! { 
	println!("Hello World{}", "!");
    blog_os::init();
    
    // trigger a breakpoint interrupt
    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    if true { panic!("Some panic message"); }
    loop{}
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}