use core::{
    alloc::Layout,
    marker::Unsize,
    mem::MaybeUninit,
    ops::{CoerceUnsized, Deref, DerefMut},
    ptr::Pointee,
};

pub type StackBox8<T> = StackBox<T, 8>;
pub type StackBox16<T> = StackBox<T, 16>;
pub type StackBox32<T> = StackBox<T, 32>;
pub type StackBox64<T> = StackBox<T, 64>;

/// A [`Box`] analogue that is exclusive to the stack, it will not overflow onto the heap.
pub struct StackBox<T: ?Sized, const STACK_SIZE: usize> {
    size: usize,
    location: *mut T,
    stack: [u8; STACK_SIZE],
}

impl<T: ?Sized, const STACK_SIZE: usize> StackBox<T, STACK_SIZE> {
    fn raw_mut(&mut self) -> &mut [u8] {
        &mut self.stack[..self.size]
    }

    fn raw(&self) -> &[u8] {
        &self.stack[..self.size]
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
        if size > STACK_SIZE {
            panic!("stackbox overflow, {} > {}", size, STACK_SIZE);
        }

        let raw_mut = &mut stack[..size];

        raw_mut.copy_from_slice(unsafe {
            core::slice::from_raw_parts(&item as *const T as *const u8, size)
        });

        let location = raw_mut.as_mut_ptr() as *mut T;

        Self {
            size,
            stack,
            location,
        }
    }

    pub fn is_heap(&self) -> bool {
        self.size > STACK_SIZE
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized, const STACK_SIZE: usize>
    CoerceUnsized<StackBox<U, STACK_SIZE>> for StackBox<T, STACK_SIZE>
{
}

impl<T: ?Sized, const STACK_SIZE: usize> Deref for StackBox<T, STACK_SIZE> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            core::ptr::from_raw_parts::<T>(self.raw().as_ptr() as *const (), self.metadata())
                .as_ref()
                .unwrap()
        }
    }
}

impl<T: ?Sized, const STACK_SIZE: usize> DerefMut for StackBox<T, STACK_SIZE> {
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
