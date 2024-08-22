use alloc::alloc::{GlobalAlloc, Layout};
use super::Locked;
use super::align_up;


pub struct Bump {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl Bump {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        Bump {
            heap_start,
            heap_end: heap_start + heap_size - 1,
            next: heap_start,
            allocations: 0,
        }
    }

}

unsafe impl GlobalAlloc for Locked<Bump> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();
        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = alloc_start + layout.size();
        if alloc_end > bump.heap_end {
            return core::ptr::null_mut()
        }
        bump.allocations += 1;
        bump.next = alloc_end;
        alloc_start as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();
        bump.allocations -= 1;

        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}