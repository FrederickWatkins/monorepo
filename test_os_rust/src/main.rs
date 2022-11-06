#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(qemu_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

#[cfg(not(test))]
use core::panic::PanicInfo;

// The entry point for the OS
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("Yabba dabba doo!");
    panic!("Test panic");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    
    println!("{}", info);
    loop {}
}