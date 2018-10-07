//! Leonardo's Pins
use atmega32u4;

define_pins! {
    /// Convenience wrapper for easy access to Arduino Leonardo pins
    ///
    /// # Example
    /// ```
    /// let dp = arduino_leonardo::Peripherals::take().unwrap();
    ///
    /// let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);
    ///
    /// // Notic that we can use `pins.ddr` for all pins.  It is generic over the
    /// // different ports.
    /// let mut led = pins.d13.into_output(pins.ddr);
    /// ```
    name: Pins,
    ddr: DDR {
        portb: atmega32u4::PORTB,
        portc: atmega32u4::PORTC,
        portd: atmega32u4::PORTD,
        porte: atmega32u4::PORTE,
        // Currently no pin is used
        // portf: atmega32u4::PORTF,
    },
    pins: {
        /// `D0` / `RX`
        ///
        /// * `RX` (UART)
        /// * `INT2`: External Interrupt
        d0:     (portd, pd2, PD2),
        /// `D1` / `TX`
        ///
        /// * `TX` (UART)
        /// * `INT3`: External Interrupt
        d1:     (portd, pd3, PD3),
        /// `D2` / `SDA`
        ///
        /// * `SDA`: i2c/twi data
        /// * `INT1`: External Interrupt
        d2:     (portd, pd1, PD1),
        /// `D3` / `SCL`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer0Pwm]
        /// * `SCL`: i2c/twi clock
        /// * `INT0`: External Interrupt
        /// * `OC0B`: Output Compare Channel `B` for Timer/Counter0
        d3:     (portd, pd0, PD0),
        /// `D4`
        d4:     (portd, pd4, PD4),
        /// `D5`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer3Pwm]
        /// * `OC3A`: Output Compare Channel `A` for Timer/Counter3
        /// * `#OC4A`: Inverted Output Compare Channel `A` for Timer/Counter4 (Not implemented)
        d5:     (portc, pc6, PC6),
        /// `D6`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer4Pwm]
        /// * `OC4D`: Output Compare Channel `D` for Timer/Counter4
        d6:     (portd, pd7, PD7),
        /// `D7`
        ///
        /// * `INT6`: External Interrupt
        d7:     (porte, pe6, PE6),
        /// `D8`
        d8:     (portb, pb4, PB4),
        /// `D9`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer1Pwm]
        /// * `OC1A`: Output Compare Channel `A` for Timer/Counter1
        /// * `#OC4B`: Inverted Output Compare Channel `B` for Timer/Counter4 (Not implemented)
        d9:     (portb, pb5, PB5),
        /// `D10`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer1Pwm]
        /// * `OC1B`: Output Compare Channel `B` for Timer/Counter1
        /// * `OC4B`: Output Compare Channel `B` for Timer/Counter4 (Not implemented)
        d10:    (portb, pb6, PB6),
        /// `D11`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer0Pwm]
        /// * `OC0A`: Output Compare Channel `B` for Timer/Counter0
        /// * `OC1C`: Output Compare Channel `C` for Timer/Counter1
        d11:    (portb, pb7, PB7),
        /// `D12`
        ///
        /// * `#OC4D`: Inverted Output Compare Channel `D` for Timer/Counter4 (Not implemented)
        d12:    (portd, pd6, PD6),
        /// `D13` / `LED_BUILTIN`
        ///
        /// * Onboard LED
        /// * **PWM**: [atmega32u4_hal::timer::Timer4Pwm]
        /// * `OC4A`: Output Compare Channel `A` for Timer/Counter4
        d13:    (portc, pc7, PC7),
        /// `RX`
        ///
        /// Led for indicating inbound data
        led_rx: (portb, pb0, PB0),
        /// `TX`
        ///
        /// Led for indicating outbound data
        led_tx: (portd, pd5, PD5),
    }
}
