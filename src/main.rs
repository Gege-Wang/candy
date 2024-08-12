#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(candy::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use candy::{hlt_loop, println};



#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    candy::init();

    #[cfg(test)]
    test_main();

    //hlt_loop();
    loop{
        
    }

    
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    candy::test_panic_handler(info);
}

