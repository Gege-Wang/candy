
use linked_list::LinkedListAllocator;
use x86_64::structures::paging::mapper::{MapToError, Mapper};
use x86_64::structures::paging::FrameAllocator;
use x86_64::structures::paging::PageTableFlags as Flags;
use x86_64::structures::paging::Size4KiB;
use x86_64::{structures::paging::Page, VirtAddr};
pub const HEAP_START: u64 = 0x4444_4444_0000;
pub const HEAP_SIZE: u64 = 100 * 1024; // 100 KiB

pub mod bump;
pub mod linked_list;
use bump::Bump;

#[global_allocator]
//static ALLOCATOR: Locked<Bump> = Locked::new(Bump::new(HEAP_START as usize, HEAP_SIZE as usize));
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page: Page<Size4KiB> = Page::containing_address(heap_start);
        let heap_end_page: Page<Size4KiB> = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = Flags::PRESENT | Flags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }
    unsafe {
        ALLOCATOR.lock().init(HEAP_START as usize, HEAP_SIZE as usize);
    }
    Ok(())
}


pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }
    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr
    } else {
        addr + align - remainder
    }
}