#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]
#![deny(unsafe_code)]

extern crate panic_msp430;

use msp430::interrupt::{CriticalSection, Mutex};
use msp430_rt::entry;
use msp430g2553::{interrupt, Peripherals};
use once_cell::unsync::OnceCell;

static PERIPHERALS: Mutex<OnceCell<Peripherals>> = Mutex::new(OnceCell::new());

#[interrupt]
fn TIMER0_A0(cs: CriticalSection) {
    PERIPHERALS.borrow(cs).get().map(|p| &p.TIMER1_A3).unwrap();
}

#[interrupt]
fn PORT1(cs: CriticalSection) {
    PERIPHERALS.borrow(cs).get().map(|p| &p.PORT_1_2).unwrap();
}

fn init(cs: CriticalSection) {
    let p = Peripherals::take().unwrap();
    PERIPHERALS.borrow(cs).set(p).map_err(|_e| {}).unwrap();
}

#[entry(interrupt_enable(pre_interrupt = init))]
fn main() -> ! {
    loop {
        
    }
}
