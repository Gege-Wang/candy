#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(candy::test_runner)]
#![reexport_test_harness_main = "test_main"]

use candy::{hlt_loop, println};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};


entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    candy::init();

    use x86_64::VirtAddr;
    use x86_64::structures::paging::PageTable;
    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let page_table = unsafe { candy::memory::active_level_4_table(physical_mem_offset) };
    for (i, entry) in page_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("l4 Entry {}: {:?}", i, entry);
            let physic = entry.frame().unwrap().start_address();
            let virt = boot_info.physical_memory_offset + physic.as_u64();
            let virt_ptr = VirtAddr::new(virt).as_mut_ptr();
            let page_table_l3: &PageTable = unsafe{& *virt_ptr};

            for (i, entry) in page_table_l3.iter().enumerate() {
                if !entry.is_unused() {
                    println!("l3 Entry {}: {:?}", i, entry);
                }
            }
        }

    }

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
