#![no_std]
#![feature(asm_experimental_arch)]

use critical_section::CriticalSection;

// For the critical-lto example, critical crate only provides acquire/release.
#[no_mangle]
pub unsafe fn acquire() -> u16 {
    internal::acquire()
}

#[no_mangle]
pub unsafe fn release(restore_state: u16) {
    internal::release(restore_state)
}

// For the critical example, the critical crate provides the free function
// as well as acquire/release.
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    unsafe {
        let restore_state = internal::acquire();
        let r = f(CriticalSection::new());
        internal::release(restore_state);
        r
    }
}

// acquire/release() are wrappers for acquire/release_internal.
// Features provided for 3 implementations:
// * MSP430
// * Cortex-M
// * Arch-agnostic dummy code
//
// In addition, an inline feature enables an inline hint, which changes
// codegen.
mod internal {
    use cfg_if::cfg_if;

    cfg_if! {
        if #[cfg(feature = "msp430")] {
            use msp430::{register, interrupt};

            #[cfg_attr(feature = "inline", inline)]
            pub unsafe fn acquire() -> u16 {
                let status = register::sr::read();
                interrupt::disable();
                // Safety: Sr is repr(C), RawRestoreState is u16, and Sr contains
                // only a single u16. This should be fine.
                core::mem::transmute(status)
            }

            #[cfg_attr(feature = "inline", inline)]
            pub unsafe fn release(restore_state: u16) {
                // Safety: Must be called w/ acquire, otherwise we could receive
                // an invalid Sr (even though internally it's a u16, not all bits
                // are actually used). It would be better to pass Sr as
                // RawRestoreState, but since we can't, this will be acceptable,
                // See acquire() for why this is safe.
                let sr: register::sr::Sr = core::mem::transmute(restore_state);
            
                if sr.gie() {
                    interrupt::enable();
                }
            }
        } else if #[cfg(feature = "cortex-m")] {
            use cortex_m::register::primask;
            use cortex_m::interrupt;

            #[cfg_attr(feature = "inline", inline)]
            pub unsafe fn acquire() -> u16 {
                let was_active = primask::read().is_active();
                interrupt::disable();
                was_active as u16
            }

            #[cfg_attr(feature = "inline", inline)]
            pub unsafe fn release(restore_state: u16) {
                if restore_state != 0 {
                    interrupt::enable()
                }
            }
        } else {
            use core::arch::asm;

            #[cfg_attr(feature = "inline", inline)]
            pub unsafe fn acquire() -> u16 {
                let fake_sr: u16 = 0;

                let sr = core::ptr::read_volatile(&fake_sr as *const u16);
                asm!("");
                sr
            }

            #[cfg_attr(feature = "inline", inline)]
            pub unsafe fn release(restore_state: u16) {
                if restore_state != 0 {
                    asm!("");
                }
            }
        }
    }
}
