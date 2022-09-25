#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]
#![cfg_attr(not(feature = "unsafe"), deny(unsafe_code))]

extern crate panic_msp430;

use cfg_if::cfg_if;

use msp430::{{interrupt as mspint}, interrupt::CriticalSection};
use msp430_rt::entry;
use msp430g2553::{interrupt, Peripherals};

cfg_if! {
    if #[cfg(not(feature = "unsafe"))] {
        use core::cell::RefCell;
        static PERIPHERALS : mspint::Mutex<RefCell<Option<Peripherals>>> =
            mspint::Mutex::new(RefCell::new(None));
    }
}

fn init(_cs: CriticalSection) {
    cfg_if! {
        if #[cfg(not(feature = "unsafe"))] {
            let p = Peripherals::take().unwrap();
        } else {
            // Safe because interrupts are disabled after a reset.
            let p = unsafe { Peripherals::steal() };
        }
    }

    let wdt = &p.WATCHDOG_TIMER;
    wdt.wdtctl.write(|w| {
       w.wdtpw().password().wdthold().set_bit()
    });

    let port_1_2 = &p.PORT_1_2;
    port_1_2.p1dir.modify(|_, w| w.p0().set_bit()
                                  .p6().set_bit());
    port_1_2.p1out.modify(|_, w| w.p0().set_bit()
                                  .p6().clear_bit());

    let clock = &p.SYSTEM_CLOCK;
    clock.bcsctl3.modify(|_, w| w.lfxt1s().lfxt1s_2());
    clock.bcsctl1.modify(|_, w| w.diva().diva_1());

    let timer = &p.TIMER0_A3;
    timer.taccr0.write(|w| w.bits(1200));
    timer.tactl.modify(|_, w| w.tassel().tassel_1()
                                .mc().mc_1());
    timer.tacctl1.modify(|_, w| w.ccie().set_bit());
    timer.taccr1.write(|w| w.bits(600));

    #[cfg(not(feature = "unsafe"))]
    mspint::free(|cs| {
        *PERIPHERALS.borrow(*cs).borrow_mut() = Some(p);
    });
}


#[entry(interrupt_enable(pre_interrupt = init))]
fn main() -> ! {

    loop {}
}

#[interrupt]
#[allow(unused_variables)]
fn TIMER0_A1(cs: CriticalSection) {
    cfg_if! {
        if #[cfg(not(feature = "unsafe"))] {
            let p_ref = PERIPHERALS.borrow(cs).borrow();
            let p = p_ref.as_ref().unwrap();
        } else {
            // Safe because msp430 disables interrupts on handler entry. Therefore the handler
            // has full control/access to peripherals without data races.
            let p = unsafe { Peripherals::steal() };
        }
    }

    let timer = &p.TIMER0_A3;
    timer.tacctl1.modify(|_, w| w.ccifg().clear_bit());

    let port_1_2 = &p.PORT_1_2;
    port_1_2.p1out.modify(|r, w| w.p0().bit(!r.p0().bit())
                                    .p6().bit(!r.p6().bit()));
}

#[no_mangle]
#[allow(unsafe_code)]
extern "C" fn abort() -> ! {
    panic!();
}
