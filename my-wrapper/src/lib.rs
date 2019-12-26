#![no_std]

use core::cell::UnsafeCell;

pub struct CriticalSection {
    pub _0: (),
}

impl CriticalSection {
    // In real code, this would be unsafe, but this is for demonstrative purposes.
    pub fn new() -> Self {
        CriticalSection { _0: () }
    }
}

pub struct Wrapper<T> {
    inner: UnsafeCell<T>,
}

impl<T> Wrapper<T> {
    pub const fn new(value: T) -> Self {
        Wrapper {
            inner: UnsafeCell::new(value),
        }
    }
}

impl<T> Wrapper<T> {
    pub fn borrow<'cs>(&'cs self, _cs: &'cs CriticalSection) -> &'cs T {
        unsafe { &*self.inner.get() }
    }
}

// This is a simplified version of bare_metal::Mutex; the dead code in min.rs exists even when
// used with the Mutex.
unsafe impl<T> Sync for Wrapper<T> where T: Send {}
