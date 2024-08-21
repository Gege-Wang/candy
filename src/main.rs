#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(candy::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use candy::{
    hlt_loop,
    memory::{self, create_example_mapping},
    println,
};
use core::panic::PanicInfo;
use x86_64::structures::paging::Translate;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");
    candy::init();

    use x86_64::VirtAddr;
    use x86_64::structures::paging::{Page, PageTableFlags as Flags};
    let physical_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut page_table = unsafe { memory::init(physical_mem_offset) };
    let mut frame_allocator = unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    let page = Page::containing_address(VirtAddr::new(0));
    // 为他找到一个虚拟地址  如果我们已经知道物理地址 我们使用地址转换 mapper 进行 物理地址到虚拟地址的转换 然后我们创建一个页表项
    //let phys_frame = frame_allocator.allocate_frame().expect("no more frames");
    let flags = Flags::PRESENT | Flags::WRITABLE;
    create_example_mapping(page, flags, &mut page_table, &mut frame_allocator);
    let addresses = [
        0x0,
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = page_table.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
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
