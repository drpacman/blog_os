#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use blog_os::hlt_loop;
mod vga_buffer;
use alloc::{
    boxed::Box, 
    vec, 
    vec::Vec, 
    rc::Rc
};

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC!!: {}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info);
}

entry_point!(kernel_main);
fn kernel_main(boot_info: &'static BootInfo) -> ! { 
    use blog_os::allocator;
    use blog_os::memory;
    use x86_64::{
        VirtAddr,
        //structures::paging::{ Page, Translate }
    };
    
	println!("Hello World! - boot info has {} regions", boot_info.memory_map.iter().count());
    blog_os::init();
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    // // write the string `New!` to the screen through the new mapping    
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    #[cfg(test)]
    test_main();

    let x = Box::new(41);
    println!("heap_value at {:p} is {}", x, *x);

    // create a dynamically sized vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p} is of size {}", vec.as_slice(), vec.len());

    // create a reference counted vector -> will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));
    // // trigger a page fault
    // let ptr = 0x207a2a as *mut u8;
    // // read from a code page
    // unsafe { let x = *ptr; }
    // println!("read worked");

    // // write to a code page
    // unsafe { *ptr = 42; }
    // println!("write worked");
    // // trigger a breakpoint interrupt
    // x86_64::instructions::interrupts::int3();
    // // trigger a divide by zero exception
    // blog_os::divide_by_zero();

    // // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };
    println!("It didn't crash");
    hlt_loop()
}


#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}