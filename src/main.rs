#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(candy::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use candy::{allocator, hlt_loop, interrupts, memory, println};
use core::panic::PanicInfo;
use candy::task::{keyboard, Task};
use candy::task::simple_executor::TaskQueue;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    candy::init();

    use x86_64::VirtAddr;
    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
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
    println!(
        "current reference count is {}",
        Rc::strong_count(&cloned_reference)
    );
    core::mem::drop(reference_count);
    println!("reference count is {}", Rc::strong_count(&cloned_reference));

    let mut task_queue = TaskQueue::new();
    task_queue.spawn(Task::new(example_async_function()));
    task_queue.spawn(Task::new(interrupts::print_keypresses()));
    task_queue.run();

    

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


async fn async_number() -> u32 {
    42
}

async fn example_async_function() {
    let number = async_number().await;
    println!("async number: {}", number);
}