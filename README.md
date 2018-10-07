# `arduino-leonardo` [![crates.io page](http://meritbadge.herokuapp.com/arduino-leonardo)](https://crates.io/crates/arduino-leonardo) [![docs.rs](https://docs.rs/arduino-leonardo/badge.svg)](https://docs.rs/arduino-leonardo)

Board support crate for *Arduino Leonardo*.  Reexports types to more closely
match the leonardo's labeling.

### Example
```rust
#![no_std]
#![no_main]
extern crate arduino_leonardo;

use arduino_leonardo::prelude::*;

#[no_mangle]
pub extern fn main() {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut delay = arduino_leonardo::Delay::new();
    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);

    let mut led0 = pins.led_rx.into_output(&mut pins.ddr);
    let mut led1 = pins.led_tx.into_output(&mut pins.ddr);
    let mut led2 = pins.d13.into_output(&mut pins.ddr);

    led0.set_high();
    led1.set_high();
    led2.set_high();

    let mut leds = [
        led0.downgrade(),
        led1.downgrade(),
        led2.downgrade(),
    ];

    loop {
        for i in 0..3 {
            leds[i].toggle();
            leds[(i+2)%3].toggle();
            delay.delay_ms(200);
        }
    }
}
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
