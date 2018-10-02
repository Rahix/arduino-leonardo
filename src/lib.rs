#![no_std]
#![feature(lang_items, unwind_attributes)]

extern crate atmega32u4;
#[macro_use]
extern crate atmega32u4_hal;

use atmega32u4_hal::delay;
pub type Delay = delay::Delay<delay::MHz16>;

pub mod pins;
pub use pins::{Pins, DDR};

pub use atmega32u4::Peripherals;

pub mod prelude {
    pub use atmega32u4_hal::prelude::*;
}

#[cfg(not(feature = "docs"))]
pub mod std {
    use super::atmega32u4;

    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {
    }

    #[lang = "panic_fmt"]
    #[unwind]
    #[no_mangle]
    pub unsafe extern "C" fn rust_begin_panic(_msg: (), _file: &'static str, _line: u32) -> ! {
        use atmega32u4_hal::prelude::*;

        let mut delay = super::Delay::new();

        let mut portb = ::core::mem::uninitialized::<atmega32u4::PORTB>().split();
        let mut portd = ::core::mem::uninitialized::<atmega32u4::PORTD>().split();

        let mut led0 = portb.pb0.into_output(&mut portb.ddr);
        let mut led1 = portd.pd5.into_output(&mut portd.ddr);

        loop {
            led0.set_high();
            delay.delay_ms(100);
            led0.set_low();

            led1.set_high();
            delay.delay_ms(100);
            led1.set_low();
        }
    }
}
