use core::alloc::Layout;

// copied from stdlib
#[cfg(all(not(feature = "std"), feature = "global_alloc_fill"))]
extern "Rust" {
    // These are the magic symbols to call the global allocator.  rustc generates
    // them to call `__rg_alloc` etc. if there is a `#[global_allocator]` attribute
    // (the code expanding that attribute macro generates those functions), or to call
    // the default implementations in libstd (`__rdl_alloc` etc. in `library/std/src/alloc.rs`)
    // otherwise.
    // The rustc fork of LLVM also special-cases these function names to be able to optimize them
    // like `malloc`, `realloc`, and `free`, respectively.
    #[rustc_allocator]
    #[rustc_allocator_nounwind]
    fn __rust_alloc(size: usize, align: usize) -> *mut u8;
    #[rustc_allocator_nounwind]
    fn __rust_dealloc(ptr: *mut u8, size: usize, align: usize);
}

// copied from stdlib
#[inline(always)]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    #[cfg(all(not(feature = "std"), feature = "global_alloc_fill"))]
    return __rust_alloc(layout.size(), layout.align());
    #[cfg(not(all(not(feature = "std"), feature = "global_alloc_fill")))]
    std::alloc::alloc(layout)
}

// copied from stdlib
#[inline(always)]
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    #[cfg(all(not(feature = "std"), feature = "global_alloc_fill"))]
    return __rust_dealloc(ptr, layout.size(), layout.align());
    #[cfg(not(all(not(feature = "std"), feature = "global_alloc_fill")))]
    std::alloc::dealloc(ptr, layout)
}
