use alloc::alloc::GlobalAlloc;
use core::{
    alloc::Layout,
    ops,
    sync::atomic::{AtomicIsize, AtomicUsize, Ordering},
};

/// An instrumenting middleware which keeps track of allocation, deallocation,
/// and reallocation requests to the underlying global allocator.
///
/// Copied out of https://crates.io/crates/stats_alloc
#[derive(Default, Debug)]
pub struct StatsAllocator<T> {
    allocations: AtomicUsize,
    deallocations: AtomicUsize,
    reallocations: AtomicUsize,
    bytes_allocated: AtomicUsize,
    bytes_deallocated: AtomicUsize,
    bytes_reallocated: AtomicIsize,
    inner: T,
}

/// Allocator statistics
#[derive(Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub struct Stats {
    /// Count of allocation operations
    pub allocations: usize,
    /// Count of deallocation operations
    pub deallocations: usize,
    /// Count of reallocation operations
    ///
    /// An example where reallocation may occur: resizing of a `Vec<T>` when
    /// its length would excceed its capacity. Excessive reallocations may
    /// indicate that resizable data structures are being created with
    /// insufficient or poorly estimated initial capcities.
    ///
    /// ```
    /// let mut x = Vec::with_capacity(1);
    /// x.push(0);
    /// x.push(1); // Potential reallocation
    /// ```
    pub reallocations: usize,
    /// Total bytes requested by allocations
    pub bytes_allocated: usize,
    /// Total bytes freed by deallocations
    pub bytes_deallocated: usize,
    /// Total of bytes requested minus bytes freed by reallocations
    ///
    /// This number is positive if the total bytes requested by reallocation
    /// operations is greater than the total bytes freed by reallocations. A
    /// positive value indicates that resizable structures are growing, while
    /// a negative value indicates that such structures are shrinking.
    pub bytes_reallocated: isize,
}

impl<T> StatsAllocator<T> {
    /// Provides access to an instrumented instance of the given global
    /// allocator.
    pub const fn new(inner: T) -> Self {
        StatsAllocator {
            allocations: AtomicUsize::new(0),
            deallocations: AtomicUsize::new(0),
            reallocations: AtomicUsize::new(0),
            bytes_allocated: AtomicUsize::new(0),
            bytes_deallocated: AtomicUsize::new(0),
            bytes_reallocated: AtomicIsize::new(0),
            inner,
        }
    }

    /// Takes a snapshot of the current view of the allocator statistics.
    pub fn stats(&self) -> Stats {
        Stats {
            allocations: self.allocations.load(Ordering::SeqCst),
            deallocations: self.deallocations.load(Ordering::SeqCst),
            reallocations: self.reallocations.load(Ordering::SeqCst),
            bytes_allocated: self.bytes_allocated.load(Ordering::SeqCst),
            bytes_deallocated: self.bytes_deallocated.load(Ordering::SeqCst),
            bytes_reallocated: self.bytes_reallocated.load(Ordering::SeqCst),
        }
    }
}

impl ops::Sub for Stats {
    type Output = Stats;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl ops::SubAssign for Stats {
    fn sub_assign(&mut self, rhs: Self) {
        self.allocations -= rhs.allocations;
        self.deallocations -= rhs.deallocations;
        self.reallocations -= rhs.reallocations;
        self.bytes_allocated -= rhs.bytes_allocated;
        self.bytes_deallocated -= rhs.bytes_deallocated;
        self.bytes_reallocated -= rhs.bytes_reallocated;
    }
}

/// A snapshot of the allocation statistics, which can be used to determine
/// allocation changes while the `Region` is alive.
#[derive(Debug)]
pub struct Region<'a, T: GlobalAlloc + 'a> {
    alloc: &'a StatsAllocator<T>,
    initial_stats: Stats,
}

impl<'a, T: GlobalAlloc + 'a> Region<'a, T> {
    /// Creates a new region using statistics from the given instrumented
    /// allocator.
    #[inline]
    pub fn new(alloc: &'a StatsAllocator<T>) -> Self {
        Region {
            alloc,
            initial_stats: alloc.stats(),
        }
    }

    /// Returns the statistics as of instantiation or the last reset.
    #[inline]
    pub fn initial(&self) -> Stats { self.initial_stats }

    /// Returns the difference between the currently reported statistics and
    /// those provided by `initial()`.
    #[inline]
    pub fn change(&self) -> Stats { self.alloc.stats() - self.initial_stats }

    /// Returns the difference between the currently reported statistics and
    /// those provided by `initial()`, resetting initial to the latest
    /// reported statistics.
    #[inline]
    pub fn change_and_reset(&mut self) -> Stats {
        let latest = self.alloc.stats();
        let diff = latest - self.initial_stats;
        self.initial_stats = latest;
        diff
    }

    /// Resets the initial initial to the latest reported statistics from the
    /// referenced allocator.
    #[inline]
    pub fn reset(&mut self) { self.initial_stats = self.alloc.stats(); }
}

unsafe impl<'a, T: GlobalAlloc + 'a> GlobalAlloc for &'a StatsAllocator<T> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { (*self).alloc(layout) }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        (*self).dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        (*self).alloc_zeroed(layout)
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        (*self).realloc(ptr, layout, new_size)
    }
}

unsafe impl<T: GlobalAlloc> GlobalAlloc for StatsAllocator<T> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocations.fetch_add(1, Ordering::SeqCst);
        self.bytes_allocated
            .fetch_add(layout.size(), Ordering::SeqCst);
        self.inner.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.deallocations.fetch_add(1, Ordering::SeqCst);
        self.bytes_deallocated
            .fetch_add(layout.size(), Ordering::SeqCst);
        self.inner.dealloc(ptr, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.allocations.fetch_add(1, Ordering::SeqCst);
        self.bytes_allocated
            .fetch_add(layout.size(), Ordering::SeqCst);
        self.inner.alloc_zeroed(layout)
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        self.reallocations.fetch_add(1, Ordering::SeqCst);
        if new_size > layout.size() {
            let difference = new_size - layout.size();
            self.bytes_allocated.fetch_add(difference, Ordering::SeqCst);
        } else if new_size < layout.size() {
            let difference = layout.size() - new_size;
            self.bytes_deallocated
                .fetch_add(difference, Ordering::SeqCst);
        }
        self.bytes_reallocated.fetch_add(
            new_size.wrapping_sub(layout.size()) as isize,
            Ordering::SeqCst,
        );
        self.inner.realloc(ptr, layout, new_size)
    }
}

pub struct DebugAllocator<A>(A);

impl<A> DebugAllocator<A> {
    pub const fn new(inner: A) -> Self { DebugAllocator(inner) }
}

unsafe impl<A: GlobalAlloc> GlobalAlloc for DebugAllocator<A> {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr = self.0.alloc(layout);
        log::trace!("Alloc {:p} ({:?})", ptr, layout);

        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        log::trace!("Free {:p} ({:?})", ptr, layout);

        self.0.dealloc(ptr, layout);
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        let new_ptr = self.0.realloc(ptr, layout, new_size);

        log::trace!(
            "Reallocated {:p} to {} bytes at {:p} ({:?})",
            ptr,
            new_size,
            new_ptr,
            layout
        );

        new_ptr
    }
}
