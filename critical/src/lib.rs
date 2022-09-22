#![no_std]
#![feature(asm_experimental_arch)]

use critical_section::CriticalSection;

// For the critical-lto example, critical crate only provides acquire/release.
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
        let r = f(CriticalSection::new());
        internal::release(0);
        r
    }
}

// acquire/release() are wrappers for acquire/release_internal.
// The same body is used for two architectures:
// * MSP430
// * Cortex-M
//
// In addition, an inline feature enables an inline hint, which changes
// codegen.
mod internal {
    use core::arch::asm;

    #[cfg_attr(feature = "inline", inline)]
    pub unsafe fn release(restore_state: u16) {
        if core::ptr::read_volatile(&restore_state as *const u16) != 0 {
            asm!("nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop
            nop");
        }
    }
}
