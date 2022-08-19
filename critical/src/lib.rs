#![no_std]
#![feature(asm_experimental_arch)]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "msp430")] {
        use msp430::{register, interrupt};
        use msp430::interrupt::CriticalSection;
    } else {
        use core::arch::asm;
        use critical_section::CriticalSection;
    }
}

cfg_if! {
    if #[cfg(feature = "msp430")] {
        #[no_mangle]
        pub unsafe fn acquire() -> u16 {
            let sr = register::sr::read().bits();
            interrupt::disable();
            // Safety: Sr is repr(C), RawRestoreState is u16, and Sr contains
            // only a single u16. This should be fine.
            core::mem::transmute(sr)
        }

        #[no_mangle]
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
    } else {
        #[no_mangle]
        pub unsafe fn acquire() -> u16 {
            let fake_sr: u16 = 0;

            let sr = core::ptr::read_volatile(&fake_sr as *const u16);
            asm!("");
            sr
        }

        #[no_mangle]
        pub unsafe fn release(restore_state: u16) {
            if restore_state != 0 {
                asm!("");
            }
        }
    }
}

pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(CriticalSection) -> R,
{
    unsafe {
        let restore_state = acquire_internal();
        let r = f(CriticalSection::new());
        release_internal(restore_state);
        r
    }
}

cfg_if! {
    if #[cfg(feature = "msp430")] {
        #[inline]
        unsafe fn acquire_internal() -> register::sr::Sr {
            let status = register::sr::read();
            interrupt::disable();
            status
        }

        #[inline]
        unsafe fn release_internal(restore_state: register::sr::Sr) {
            if restore_state.gie() {
                interrupt::enable();
            }
        }
    } else {
        #[inline]
        unsafe fn acquire_internal() -> u16 {
            let fake_sr: u16 = 0;

            let sr = core::ptr::read_volatile(&fake_sr as *const u16);
            asm!("");
            sr
        }

        #[no_mangle]
        pub unsafe fn release_internal(restore_state: u16) {
            if restore_state != 0 {
                asm!("");
            }
        }
    }
}
