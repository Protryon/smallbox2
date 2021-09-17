#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(ptr_metadata)]
#![no_std]
#![cfg_attr(
    all(not(feature = "std"), feature = "global_alloc_fill"),
    feature(rustc_attrs)
)]

#[cfg(feature = "std")]
#[doc(hidden)]
extern crate std;

#[cfg(any(feature = "std", feature = "global_alloc_fill"))]
mod smallbox;
#[cfg(any(feature = "std", feature = "global_alloc_fill"))]
pub use smallbox::*;

mod stackbox;
pub use stackbox::*;

mod global_alloc;
