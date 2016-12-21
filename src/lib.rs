//! Memory map for nRF51 microcontrollers

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate volatile_register;

#[allow(missing_docs)]
pub mod gpio;
#[allow(missing_docs)]
pub mod timer;

use gpio::Gpio;
use timer::Timer;

// const POWER: usize = 0x40000000;
// const CLOCK: usize = 0x40000000;
// const MPU: usize = 0x40000000;
// const AMLI: usize = 0x40000000;
// const RADIO: usize = 0x40001000;
// const UART0: usize = 0x40002000;
// const SPI0: usize = 0x40003000;
// const TWI0: usize = 0x40003000;
// const SPI1: usize = 0x40004000;
// const TWI1: usize = 0x40004000;
// const SPIS1: usize = 0x40004000;
// const SPIM1: usize = 0x40004000;
// const GPIOTE: usize = 0x40006000;
// const ADC: usize = 0x40007000;
const TIMER0: usize = 0x40008000;
const TIMER1: usize = 0x40009000;
const TIMER2: usize = 0x4000a000;
// const RTC0: usize = 0x4000b000;
// const TEMP: usize = 0x4000c000;
// const RNG: usize = 0x4000d000;
// const ECB: usize = 0x4000e000;
// const AAR: usize = 0x4000f000;
// const CCM: usize = 0x4000f000;
// const WDT: usize = 0x40010000;
// const RTC1: usize = 0x40011000;
// const QDEC: usize = 0x40012000;
// const LPCOMP: usize = 0x40013000;
// const SWI: usize = 0x40014000;
// const NVMC: usize = 0x4001e000;
// const PPI: usize = 0x4001f000;
// const FICR: usize = 0x10000000;
// const UICR: usize = 0x10001000;
const GPIO: usize = 0x50000000;

/// GPIO register block (&'static)
pub fn gpio() -> &'static Gpio {
    unsafe { deref(GPIO) }
}

/// GPIO register block (&'static mut)
pub unsafe fn gpio_mut() -> &'static mut Gpio {
    deref_mut(GPIO)
}

/// TIMER0 register block (&'static)
pub fn timer0() -> &'static Timer {
    unsafe { deref(TIMER0) }
}

/// TIMER0 register block (&'static mut)
pub unsafe fn timer0_mut() -> &'static mut Timer {
    deref_mut(TIMER0)
}

/// TIMER1 register block (&'static)
pub fn timer1() -> &'static Timer {
    unsafe { deref(TIMER1) }
}

/// TIMER1 register block (&'static mut)
pub unsafe fn timer1_mut() -> &'static mut Timer {
    deref_mut(TIMER1)
}

/// TIMER2 register block (&'static)
pub fn timer2() -> &'static Timer {
    unsafe { deref(TIMER2) }
}

/// TIMER2 register block (&'static mut)
pub unsafe fn timer2_mut() -> &'static mut Timer {
    deref_mut(TIMER2)
}

unsafe fn deref<T>(address: usize) -> &'static T {
    &*(address as *const T)
}

unsafe fn deref_mut<T>(address: usize) -> &'static mut T {
    &mut *(address as *mut T)
}
