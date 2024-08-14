//creating a tss
//creating a gdt
//loading the gdt
//loading the tr
//double exception switch the stack
use lazy_static::lazy_static;
use x86_64::instructions::segmentation::{Segment, CS};
use x86_64::instructions::tables::load_tss;
#[warn(static_mut_refs)]
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

lazy_static! {
    static ref GDT:(GlobalDescriptorTable, SegmentSelector, SegmentSelector) = {
        let mut gdt = GlobalDescriptorTable::new();
        // add a code segment descriptor
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        // add a tss descriptor
        let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
        (gdt, code_selector, tss_selector)
    };
}
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            unsafe {
                let stack_start = VirtAddr::from_ptr(&STACK);
                let stack_end = stack_start + STACK_SIZE as u64;
                stack_end
            }
        };
        tss
    };
}

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(SegmentSelector::from(GDT.1));
        load_tss(SegmentSelector::from(GDT.2));
    }
}
