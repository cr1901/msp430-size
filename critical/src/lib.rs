#![no_std]
#![feature(asm_experimental_arch)]

use cfg_if::cfg_if;

// critical crate provides the only provides acquire/release.
#[no_mangle]
pub unsafe fn acquire() -> u16 {
    acquire_internal()
}

#[no_mangle]
pub unsafe fn release(restore_state: u16) {
    release_internal(restore_state)
}

// critical crate provides the free function as well as acquire/release.
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
        use msp430::{register, interrupt};
        use msp430::interrupt::CriticalSection;

        #[cfg_attr(feature = "inline", inline)]
        unsafe fn acquire_internal() -> u16 {
            let status = register::sr::read();
            interrupt::disable();
            // Safety: Sr is repr(C), RawRestoreState is u16, and Sr contains
            // only a single u16. This should be fine.
            core::mem::transmute(status)
        }

        #[cfg_attr(feature = "inline", inline)]
        unsafe fn release_internal(restore_state: u16) {
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
        use core::arch::asm;
        use critical_section::CriticalSection;

        #[cfg_attr(feature = "inline", inline)]
        unsafe fn acquire_internal() -> u16 {
            let fake_sr: u16 = 0;

            let sr = core::ptr::read_volatile(&fake_sr as *const u16);
            asm!("");
            sr
        }

        #[cfg_attr(feature = "inline", inline)]
        pub unsafe fn release_internal(restore_state: u16) {
            if restore_state != 0 {
                asm!("");
            }
        }
    }
}
