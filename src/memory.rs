use x86_64::{
    structures::paging::{OffsetPageTable, PageTable}, PhysAddr, VirtAddr
};

pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;
    let (active_level_4_table_frame, _) = Cr3::read();
    let active_level_4_table_ptr = (physical_memory_offset + active_level_4_table_frame.start_address().as_u64()).as_mut_ptr();
    &mut *active_level_4_table_ptr
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::structures::paging::mapper::Mapper;
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_frame;
    for &index in &table_indexes {
        let Virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr = Virt.as_mut_ptr();

        let table = unsafe { 
            & *(table_ptr as *const PageTable) 
        };
        
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(_) => return None,
        };
    }
    Some(frame.start_address() + u64::from(addr.page_offset()))
}