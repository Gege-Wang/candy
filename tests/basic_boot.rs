#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(candy::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use candy::println;
use core::panic::PanicInfo;
entry_point!(main);
fn main(boot_info: &'static BootInfo) -> ! {
    candy::init();
    test_main();
    candy::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    candy::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
