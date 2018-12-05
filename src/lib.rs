//! Board Support Crate for Arduino Leonardo
//! ========================================
//!
//! Reexports with the naming scheme from leonardo's
//! labels.
//!
//! Also contains a panic handler that rapidly blinks RX and TX leds.
//!
//! ## Example
//! ```rust
//! #![no_std]
//! #![no_main]
//! extern crate arduino_leonardo;
//!
//! use arduino_leonardo::prelude::*;
//!
//! #[no_mangle]
//! pub extern fn main() {
//!     let dp = arduino_leonardo::Peripherals::take().unwrap();
//!
//!     let mut delay = arduino_leonardo::Delay::new();
//!     let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);
//!
//!     let mut led0 = pins.led_rx.into_output(&mut pins.ddr);
//!     let mut led1 = pins.led_tx.into_output(&mut pins.ddr);
//!     let mut led2 = pins.d13.into_output(&mut pins.ddr);
//!
//!     led0.set_high();
//!     led1.set_high();
//!     led2.set_high();
//!
//!     let mut leds = [
//!         led0.downgrade(),
//!         led1.downgrade(),
//!         led2.downgrade(),
//!     ];
//!
//!     loop {
//!         for i in 0..3 {
//!             leds[i].toggle();
//!             leds[(i+2)%3].toggle();
//!             delay.delay_ms(200);
//!         }
//!     }
//! }
//! ```
#![no_std]
#![cfg_attr(not(feature = "docs"), feature(lang_items, panic_handler))]
#![cfg_attr(feature = "docs", feature(extern_prelude))]

extern crate atmega32u4;
#[macro_use]
extern crate atmega32u4_hal;

use atmega32u4_hal::delay;
/// Delay type that is fixed to the leonardo's 16 MHz clock
pub type Delay = delay::Delay<delay::MHz16>;

pub mod pins;
pub use pins::{Pins, DDR};

/// ATmega32U4 peripherals
pub use atmega32u4::Peripherals;

/// Prelude from `atmega32u4-hal`
pub mod prelude {
    pub use atmega32u4_hal::prelude::*;
}

#[cfg(not(feature = "docs"))]
pub mod std {
    use super::atmega32u4;

    #[lang = "eh_personality"]
    pub unsafe extern "C" fn rust_eh_personality(
        _state: (),
        _exception_object: *mut (),
        _context: *mut (),
    ) -> () {
    }

    #[panic_handler]
    fn panic(_info: &::core::panic::PanicInfo) -> ! {
        use atmega32u4_hal::prelude::*;

        let mut delay = super::Delay::new();

        let (mut portb, mut portd) = unsafe {
            (
                ::core::mem::uninitialized::<atmega32u4::PORTB>().split(),
                ::core::mem::uninitialized::<atmega32u4::PORTD>().split(),
            )
        };

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
