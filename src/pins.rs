use atmega32u4;
use atmega32u4_hal::port;

pub type LedBuiltin = port::portc::PC7<port::Output>;
pub type LedBuiltinRX = port::portb::PB0<port::Output>;
pub type LedBuiltinTX = port::portd::PD5<port::Output>;

pub struct Pins {
    pub d0: port::portd::PD2<port::Input>,
    pub d1: port::portd::PD3<port::Input>,
    pub d2: port::portd::PD1<port::Input>,
    pub d3: port::portd::PD0<port::Input>,
    pub d4: port::portd::PD4<port::Input>,
    pub d5: port::portc::PC6<port::Input>,
    pub d6: port::portd::PD7<port::Input>,
    pub d7: port::porte::PE6<port::Input>,
    pub d8: port::portb::PB4<port::Input>,
    pub d9: port::portb::PB5<port::Input>,
    pub d10: port::portb::PB6<port::Input>,
    pub d11: port::portb::PB7<port::Input>,
    pub d12: port::portd::PD6<port::Input>,
    pub d13: port::portc::PC7<port::Input>,
    pub led_rx: port::portb::PB0<port::Input>,
    pub led_tx: port::portd::PD5<port::Input>,
}

impl Pins {
    pub fn new(
        port_b: atmega32u4::PORTB,
        port_c: atmega32u4::PORTC,
        port_d: atmega32u4::PORTD,
        port_e: atmega32u4::PORTE,
    ) -> (Pins, port::portb::DDR, port::portc::DDR, port::portd::DDR, port::porte::DDR) {
        use atmega32u4_hal::port::PortExt;

        let portb = port_b.split();
        let portc = port_c.split();
        let portd = port_d.split();
        let porte = port_e.split();

        (
            Pins {
                d0: portd.pd2,
                d1: portd.pd3,
                d2: portd.pd1,
                d3: portd.pd0,
                d4: portd.pd4,
                d5: portc.pc6,
                d6: portd.pd7,
                d7: porte.pe6,
                d8: portb.pb4,
                d9: portb.pb5,
                d10: portb.pb6,
                d11: portb.pb7,
                d12: portd.pd6,
                d13: portc.pc7,
                led_rx: portb.pb0,
                led_tx: portd.pd5,
            },
            portb.ddr,
            portc.ddr,
            portd.ddr,
            porte.ddr,
        )
    }
}
