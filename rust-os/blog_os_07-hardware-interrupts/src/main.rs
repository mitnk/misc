#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use blog_os::{print, println};
use blog_os::vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    blog_os::init();
    vga_buffer::init();

    println!("Hello World{}", "!");
    println!("another line");

    // uncomment to see timer
    /*
    loop {
        // slowdown a bit
        for _ in 0..100000 {}
        use blog_os::print;
        print!("-");        // new
    }*/

    print!(".EOF.");
    #[cfg(test)]
    test_main();

    blog_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
