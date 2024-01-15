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
    use blog_os::{ exit_qemu, QemuExitCode };
    println!("PANIC!!: {}", info);
    exit_qemu(QemuExitCode::Failed);
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
    
    #[cfg(test)]
    test_main();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    // // trigger a breakpoint interrupt
    // x86_64::instructions::interrupts::int3();
    // // trigger a divide by zero exception
    // blog_os::divide_by_zero();

    // // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };

    if true { panic!("Some panic message"); }
    loop{}
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}