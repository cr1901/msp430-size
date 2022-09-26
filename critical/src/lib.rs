#![no_std]
#![feature(asm_experimental_arch)]

// For the critical-lto example, critical crate only provides acquire/release.
#[no_mangle]
pub unsafe fn release() {
    internal::release()
}

// For the critical example, the critical crate provides the free function
// as well as acquire/release.
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    unsafe {
        let r = f();
        internal::release();
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
    pub unsafe fn release() {
        let fake_reg: u16 = 0;
        if core::ptr::read_volatile(&fake_reg as *const u16) != 0 {
            asm!(
                "nop
                nop
                nop
                nop
                nop
                nop
                nop
                nop
                nop
                nop"
            );
        }
    }
}
