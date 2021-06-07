use alloc::alloc::GlobalAlloc;
use core::{
    alloc::Layout,
    ops::{Deref, DerefMut},
};
use crate::stats_allocator::StatsAllocator;

#[derive(Debug, Default)]
pub struct Allocator<A>(StatsAllocator<A>);

impl<A> Allocator<A> {
    pub const fn new(inner: A) -> Self { Allocator(StatsAllocator::new(inner)) }
}

unsafe impl<A: GlobalAlloc> GlobalAlloc for Allocator<A> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = self.0.alloc(layout);
        log::debug!(
            "Alloc {:p}, layout = {:?}, stats = {:?}",
            ptr,
            layout,
            self.0.stats()
        );

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        log::debug!(
            "Free {:p}, layout = {:?}, stats = {:?}",
            ptr,
            layout,
            self.0.stats()
        );

        self.0.dealloc(ptr, layout);
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        let new_ptr = self.0.realloc(ptr, layout, new_size);

        log::debug!(
            "Realloc {:p} to {} bytes at {:p}, layout = {:?}, stats = {:?}",
            ptr,
            new_size,
            new_ptr,
            layout,
            self.0.stats(),
        );

        new_ptr
    }
}

impl<A> Deref for Allocator<A> {
    type Target = StatsAllocator<A>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<A> DerefMut for Allocator<A> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
