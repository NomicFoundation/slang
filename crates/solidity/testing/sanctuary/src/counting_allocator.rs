use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
pub struct CountingAlloc;

#[cfg_attr(feature = "mem_profiler", global_allocator)]
#[allow(dead_code)]
static COUNTING_ALLOCATOR: CountingAlloc = CountingAlloc;

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);
static DEALLOCATED: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), Relaxed);
        }
        ret
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        DEALLOCATED.fetch_add(layout.size(), Relaxed);
    }
}

impl CountingAlloc {
    #[allow(dead_code)]
    pub fn reset() {
        DEALLOCATED.store(0, Relaxed);
        ALLOCATED.store(0, Relaxed);
    }

    pub fn allocated() -> usize {
        ALLOCATED.load(Relaxed)
    }

    #[allow(dead_code)]
    pub fn deallocated() -> usize {
        DEALLOCATED.load(Relaxed)
    }
}
