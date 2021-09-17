use core::{
    alloc::Layout,
    marker::Unsize,
    mem::MaybeUninit,
    ops::{CoerceUnsized, Deref, DerefMut},
    ptr::Pointee,
};

pub type SmallBox8<T> = SmallBox<T, 8>;
pub type SmallBox16<T> = SmallBox<T, 16>;
pub type SmallBox32<T> = SmallBox<T, 32>;
pub type SmallBox64<T> = SmallBox<T, 64>;

/// A [`Box`] analogue that will attempt to allocate to the stack with heap fallback
pub struct SmallBox<T: ?Sized, const STACK_SIZE: usize> {
    size: usize,
    location: *mut T,
    stack: [u8; STACK_SIZE],
    alloced: *mut [u8],
}

impl<T: ?Sized, const STACK_SIZE: usize> SmallBox<T, STACK_SIZE> {
    fn raw_mut(&mut self) -> &mut [u8] {
        if self.alloced.is_null() {
            &mut self.stack[..self.size]
        } else {
            unsafe { self.alloced.as_mut().unwrap() }
        }
    }

    fn raw(&self) -> &[u8] {
        if self.alloced.is_null() {
            &self.stack[..self.size]
        } else {
            unsafe { self.alloced.as_ref().unwrap() }
        }
    }

    fn metadata(&self) -> <T as Pointee>::Metadata {
        core::ptr::metadata(self.location)
    }

    pub fn new(item: T) -> Self
    where
        T: Sized,
    {
        let layout = Layout::for_value(&item);
        let size = layout.size();
        let mut stack: [u8; STACK_SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
        let mut alloced =
            unsafe { core::slice::from_raw_parts_mut(core::ptr::null_mut::<u8>(), 0) as *mut [u8] };
        if size > STACK_SIZE {
            alloced = unsafe {
                core::slice::from_raw_parts_mut(crate::global_alloc::alloc(layout), size)
            };
            if alloced.is_null() {
                panic!("alloc failed in SmallBox");
            }
        }

        let raw_mut = if alloced.is_null() {
            &mut stack[..size]
        } else {
            unsafe { alloced.as_mut().unwrap() }
        };

        raw_mut.copy_from_slice(unsafe {
            core::slice::from_raw_parts(&item as *const T as *const u8, size)
        });

        let location = raw_mut.as_mut_ptr() as *mut T;

        Self {
            size,
            stack,
            alloced,
            location,
        }
    }

    pub fn is_heap(&self) -> bool {
        self.size > STACK_SIZE
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized, const STACK_SIZE: usize>
    CoerceUnsized<SmallBox<U, STACK_SIZE>> for SmallBox<T, STACK_SIZE>
{
}

impl<T: ?Sized, const STACK_SIZE: usize> Deref for SmallBox<T, STACK_SIZE> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            core::ptr::from_raw_parts::<T>(self.raw().as_ptr() as *const (), self.metadata())
                .as_ref()
                .unwrap()
        }
    }
}

impl<T: ?Sized, const STACK_SIZE: usize> DerefMut for SmallBox<T, STACK_SIZE> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            core::ptr::from_raw_parts_mut::<T>(
                self.raw_mut().as_mut_ptr() as *mut (),
                self.metadata(),
            )
            .as_mut()
            .unwrap()
        }
    }
}

impl<T: ?Sized, const STACK_SIZE: usize> Drop for SmallBox<T, STACK_SIZE> {
    fn drop(&mut self) {
        if !self.alloced.is_null() {
            let layout = Layout::for_value(self.deref());
            unsafe { crate::global_alloc::dealloc(self.alloced as *mut u8, layout) };
        }
    }
}
