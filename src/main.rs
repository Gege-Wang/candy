#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(candy::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use alloc::rc::Rc;
use bootloader::{entry_point, BootInfo};
use candy::{
    hlt_loop,
    memory,
    println,
    allocator
};
use core::panic::PanicInfo;


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    candy::init();

    use x86_64::VirtAddr;
    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(41);
    println!("heap_value at {:p}", x);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_count = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_count.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_count);
    println!("reference count is {}", Rc::strong_count(&cloned_reference));


    #[cfg(test)]
    test_main();

    hlt_loop();
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
