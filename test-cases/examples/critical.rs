#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]
#![feature(asm_experimental_arch)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "msp430") ] {
        extern crate msp430g2553;
        extern crate panic_msp430;
        use msp430_rt::entry;
    } else if #[cfg(target_arch = "arm") ] {
        use panic_halt as _;
        use cortex_m_rt::entry;
    }
}

extern crate critical;

use core::arch::asm;
use critical_section::{CriticalSection, Mutex};
use once_cell::unsync::OnceCell;

cfg_if! {
    if #[cfg(not(feature = "use-extern-cs"))] {
        use critical::free;
    } else {
        pub fn free<F, R>(f: F) -> R
        where
            F: FnOnce() -> R,
        {
            extern "Rust" {
                fn release();
            }    

            unsafe {
                let r = f();
                release();
                r
            }
        }
    }
}

static PERIPHERALS: Mutex<OnceCell<()>> = Mutex::new(OnceCell::new());

#[entry]
fn main() -> ! {
    free(|| {
        let _ = PERIPHERALS
            .borrow(unsafe { CriticalSection::new() })
            .set(())?;

        Ok::<(),()>(())
    }).unwrap();

    loop {
        // start_timer().unwrap();
        unsafe { asm!(""); }
    }
}
