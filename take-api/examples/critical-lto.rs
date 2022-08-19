#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]
#![feature(asm_experimental_arch)]

// Required for linking.
extern crate msp430g2553;
extern crate critical;
extern crate panic_msp430;

use core::arch::asm;

use critical_section::CriticalSection;

// Required for generating actual main.
use msp430_rt::entry;

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

pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    unsafe fn acquire() -> u16 {
        let fake_sr: u16 = 0;

        // Do NOT optimize out the read.
        let sr = core::ptr::read_volatile(&fake_sr as *const u16);
        asm!("");
        sr
    }

    unsafe fn release(restore_state: u16) {    
        if restore_state != 0 {
            asm!("");
        }
    }

    unsafe {
        let restore_state = acquire();
        let r = f(CriticalSection::new());
        release(restore_state);
        r
    }
}

// Safety: Single-threaded program.
static mut PERIPHERALS: Option<()> = None;

#[entry]
fn main() -> ! {
    unsafe {
        let set_once = &mut PERIPHERALS;
        *set_once = Some(());
    }

    loop {
        if let Err(()) = start_timer() {
            unsafe { asm!(""); }
        }
    }
}

fn start_timer() -> Result<(), ()> {
    #[cfg(feature = "use-extern-cs")]
    {
        free_extern(|_| {
            let option = unsafe { 
                &mut PERIPHERALS
            };

            if let None = option {
                return Err(());
            }

            unsafe { asm!(""); }
            Ok(())
        })
    }

    #[cfg(not(feature = "use-extern-cs"))]
    {
        free(|_| {
            let option = unsafe { 
                &mut PERIPHERALS
            };

            if let None = option {
                return Err(());
            }

            unsafe { asm!(""); }
            Ok(())
        })
    }
}
