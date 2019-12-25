#![no_std]
#![no_main]
#![feature(lang_items, start)]
#![feature(abi_msp430_interrupt)]

extern crate panic_msp430;

use core::cell::UnsafeCell;
use core::cell::RefCell;

use msp430_rt::entry;

#[allow(unused_imports)]
use msp430g2553;

#[cfg(feature = "bare_metal")]
use bare_metal;

/// Critical section token
///
/// Indicates that you are executing code within a critical section
pub struct CriticalSection {
    _0: (),
}

impl CriticalSection {
    /// Creates a critical section token
    ///
    /// This method is meant to be used to create safe abstractions rather than
    /// meant to be directly used in applications.
    pub unsafe fn new() -> Self {
        CriticalSection { _0: () }
    }
}

/// A "mutex" based on critical sections
///
/// # Safety
///
/// **This Mutex is only safe on single-core systems.**
///
/// On multi-core systems, a `CriticalSection` **is not sufficient** to ensure exclusive access.
pub struct Mutex<T> {
    inner: UnsafeCell<T>,
}

impl<T> Mutex<T> {
    /// Creates a new mutex
    pub const fn new(value: T) -> Self {
        Mutex {
            inner: UnsafeCell::new(value),
        }
    }
}

impl<T> Mutex<T> {
    /// Borrows the data for the duration of the critical section
    pub fn borrow<'cs>(&'cs self, _cs: &'cs CriticalSection) -> &'cs T {
        unsafe { &*self.inner.get() }
    }
}

// NOTE A `Mutex` can be used as a channel so the protected data must be `Send`
// to prevent sending non-Sendable stuff (e.g. access tokens) across different
// execution contexts (e.g. interrupts)
unsafe impl<T> Sync for Mutex<T> where T: Send {}

#[cfg(not(feature = "bare_metal"))]
static PERIPHERALS : Mutex<RefCell<Option<u8>>> =
    Mutex::new(RefCell::new(None));

#[cfg(feature = "bare_metal")]
static PERIPHERALS : bare_metal::Mutex<RefCell<Option<u8>>> =
    bare_metal::Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    #[cfg(not(feature = "bare_metal"))]
    let _ = PERIPHERALS.borrow(unsafe { &CriticalSection::new() }).borrow_mut();
    #[cfg(not(feature = "bare_metal"))]
    let _ = PERIPHERALS.borrow(unsafe { &CriticalSection::new() }).borrow();

    #[cfg(feature = "bare_metal")]
    let _ = PERIPHERALS.borrow(unsafe { &bare_metal::CriticalSection::new() }).borrow_mut();
    #[cfg(feature = "bare_metal")]
    let _ = PERIPHERALS.borrow(unsafe { &bare_metal::CriticalSection::new() }).borrow();

    loop { }
}
