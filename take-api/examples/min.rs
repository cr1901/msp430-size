#![no_std]
#![no_main]
#![feature(lang_items, start)]
#![feature(abi_msp430_interrupt)]

extern crate panic_msp430;

use core::cell::RefCell;

use msp430_rt::entry;

#[allow(unused_imports)]
use msp430g2553;

use my_wrapper;

static PERIPHERALS : my_wrapper::Wrapper<RefCell<Option<u8>>> =
    my_wrapper::Wrapper::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    #[cfg(feature = "trigger-regress")]
    {
        let _ = PERIPHERALS.borrow(&my_wrapper::CriticalSection::new()).borrow_mut();
        let _ = PERIPHERALS.borrow(&my_wrapper::CriticalSection::new()).borrow();
    }

    #[cfg(not(feature = "trigger-regress"))]
    {
        let _ = PERIPHERALS.borrow(&my_wrapper::CriticalSection { _0 : () }).borrow_mut();
        let _ = PERIPHERALS.borrow(&my_wrapper::CriticalSection { _0 : () }).borrow();
    }

    loop { }
}
