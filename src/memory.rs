use bootloader::bootinfo::MemoryRegionType;
use x86_64::structures::paging::mapper::OffsetPageTable;
use x86_64::structures::paging::PhysFrame;
use x86_64::{structures::paging::PageTable, PhysAddr, VirtAddr};
/// should only be called once from `init` function, because it can easily lead to
/// aliased mutable references when called multiple times, which can cause undefined behavior.
/// so it can not be pub.
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;
    let (active_level_4_table_frame, _) = Cr3::read();
    let active_level_4_table_ptr =
        (physical_memory_offset + active_level_4_table_frame.start_address().as_u64()).as_mut_ptr();
    &mut *active_level_4_table_ptr
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}

use x86_64::structures::paging::Size4KiB;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags as Flags};
/// 我不需要知道物理页框，分配器回去调整。
/// 我需要知道虚拟地址，标志位以及什么样的分配器。 mapper 和分配器有什么区别呢？
pub fn create_example_mapping(
    page: Page,
    flags: Flags,
    mapper: &mut OffsetPageTable,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    //  我现在需要为一个确定的物理地址建立映射， 我需要一个特定的物理页框
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));

    //  what is mapper?
    let map_result = unsafe { mapper.map_to(page, frame, flags, frame_allocator) };
    // 在更改页表之后不要忘记对 TLB  进行刷新
    map_result.expect("map_to failed").flush();
}

pub struct BootInfoFrameAllocator {
    memory_map: &'static bootloader::bootinfo::MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static bootloader::bootinfo::MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 4,
        }
    }
    //  返回一个迭代器，所以我们能够知道下一个我们想要的可用内存在哪里。
    fn usable_frame(&self) -> impl Iterator<Item = PhysFrame> {
        let usable_regions = self
            .memory_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addressed = addr_ranges.flat_map(|r| r.step_by(4096));
        frame_addressed.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = self.usable_frame().nth(self.next);
        self.next += 1;
        frame
    }
}
