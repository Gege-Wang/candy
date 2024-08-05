#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(candy::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use candy::println;

// use vga_buffer::WRITER;
// mod vga_buffer;
// mod serial;
// use crate::vga_buffer::BUFFER_HEIGHT;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // vga_buffer::WRITER.lock().write_str("Hello World!").unwrap();
    // write!(vga_buffer::WRITER.lock(), "Lock Hello World!").unwrap();
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}

    
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    candy::test_panic_handler(info);
}





// #[test_case]
// fn trivial_assertion() {
//     assert_eq!(2, 2);
// }

// #[test_case]
// fn test_println_single() {
//     println!("Simple test to println something!");
// }

// #[test_case]
// fn test_println_many() {
//     for _ in 0..200 {
//         println!("test println many");
//     }
// }

// #[test_case]
// fn test_println_output() {
//     let s = "The quick brown fox jumps over the lazy dog";
//     println!("{}", s);
//     for (i, c) in s.chars().enumerate() {
//         let schar = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
//         assert_eq!(char::from(schar.ascii_character), c);
//     }
// }

