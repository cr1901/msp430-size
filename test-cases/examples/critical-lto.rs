#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]
#![feature(asm_experimental_arch)]

use cfg_if::cfg_if;
use core::arch::asm;
use critical_section::CriticalSection;

cfg_if! {
    if #[cfg(target_arch = "msp430") ] {
        // Required for linking.
        extern crate msp430g2553;
        extern crate critical;
        extern crate panic_msp430;
        // Required for generating actual main.
        use msp430_rt::entry;
    } else if #[cfg(target_arch = "arm") ] {
        extern crate critical;

        use panic_halt as _;
        use cortex_m_rt::entry;
    }
}

pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    cfg_if! {
        // Defer to critical crate for acquire/release.
        if #[cfg(feature = "use-extern-cs")] {
            extern "Rust" {
                fn acquire() -> u16;
                fn release(restore_state: u16);
            }
        } else {
            // Internally-provided acquire/release.
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

// #[inline(never)]
fn start_timer() -> Result<(), ()> {
    // free(|_| {
        let option = unsafe { 
            &mut PERIPHERALS
        };

        if let None = option {
            return Err(());
        }

        unsafe { asm!(""); }
        Ok(())
    // })
}

// just build-example "critical-lto" "" "msp430-none-elf" 
// just build-example "critical-lto" "use-extern-cs" "msp430-none-elf"
// returns different sizes using nightly-2022-08-10 (if free not commented out)
// returns same size on nightly-2022-09-21
//
// If #[inline(never)] used on start_timer(), size changes compared to not
// using #[inline(never)], even on nightly-2022-09-21 (incl. if free commented
// out). Basically, the if inside the loop {} may or may not be optimized
// out depending on the complexity of start_timer()'s body (even if #[inline(never)])
//
// See also: https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=884d660cb609125f2e515919a5e3ea43
// (Comment out first unsafe block to change codegen. Codegen may vary from
// this example due to LTO).
