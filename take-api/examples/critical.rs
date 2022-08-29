#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "msp430") ] {
        extern crate msp430g2553;
        extern crate panic_msp430;
        use msp430::asm::barrier;
        use msp430_rt::entry;
    } else if #[cfg(target_arch = "arm") ] {
        use panic_halt as _;
        use cortex_m::asm::dsb as barrier;
        use cortex_m_rt::entry;
    }
}

extern crate critical;
use critical_section::{CriticalSection, Mutex};
use once_cell::unsync::OnceCell;

#[cfg(not(feature = "use-extern-cs"))]
use critical::free;

pub fn free_extern<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    extern "Rust" {
        fn acquire() -> u16;
        fn release(restore_state: u16);
    }    

    unsafe {
        let restore_state = acquire();
        let r = f(CriticalSection::new());
        release(restore_state);
        r
    }
}

static PERIPHERALS: Mutex<OnceCell<()>> = Mutex::new(OnceCell::new());

#[entry]
fn main() -> ! {
    let _ = PERIPHERALS
        .borrow(unsafe { CriticalSection::new() })
        .set(());

    start_timer().unwrap();
    loop {
        // start_timer().unwrap();
        barrier();
    }
}

fn start_timer() -> Result<(), ()> {
    #[cfg(feature = "use-extern-cs")]
    {
        free_extern(|_| {
            let _ = &PERIPHERALS
                .borrow(unsafe { CriticalSection::new() })
                .get()
                // .unwrap(); // If unwrap() is used instead, codegen is identical.
                .ok_or(())?; 

            barrier();
            Ok(())
        })
    }

    #[cfg(not(feature = "use-extern-cs"))]
    {
        free(|_| {
            let _ = &PERIPHERALS
                .borrow(unsafe { CriticalSection::new() })
                .get()
                // .unwrap(); // If unwrap() is used instead, codegen is identical.
                .ok_or(())?;

            barrier();
            Ok(())
        })
    }
}
