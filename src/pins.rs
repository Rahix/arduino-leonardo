use atmega32u4;
use atmega32u4_hal::port;
use atmega32u4_hal::port::mode;

pub type LedBuiltin = port::portc::PC7<mode::io::Output>;
pub type LedBuiltinRX = port::portb::PB0<mode::io::Output>;
pub type LedBuiltinTX = port::portd::PD5<mode::io::Output>;

macro_rules! ddr_impl {
    ($($portx:ident),+) => {
        /// Generic DDR type that can be used for all ports
        pub struct DDR {
            $(
                $portx: port::$portx::DDR,
            )+
        }

        $(
            impl port::$portx::PortDDR for DDR {
                fn ddr(&mut self) -> &atmega32u4::$portx::DDR {
                    self.$portx.ddr()
                }
            }
        )+
    }
}

ddr_impl!(portb, portc, portd, porte, portf);

pub struct Pins {
    pub d0: port::portd::PD2<mode::io::Input<mode::io::Floating>>,
    pub d1: port::portd::PD3<mode::io::Input<mode::io::Floating>>,
    pub d2: port::portd::PD1<mode::io::Input<mode::io::Floating>>,
    pub d3: port::portd::PD0<mode::io::Input<mode::io::Floating>>,
    pub d4: port::portd::PD4<mode::io::Input<mode::io::Floating>>,
    pub d5: port::portc::PC6<mode::io::Input<mode::io::Floating>>,
    pub d6: port::portd::PD7<mode::io::Input<mode::io::Floating>>,
    pub d7: port::porte::PE6<mode::io::Input<mode::io::Floating>>,
    pub d8: port::portb::PB4<mode::io::Input<mode::io::Floating>>,
    pub d9: port::portb::PB5<mode::io::Input<mode::io::Floating>>,
    pub d10: port::portb::PB6<mode::io::Input<mode::io::Floating>>,
    pub d11: port::portb::PB7<mode::io::Input<mode::io::Floating>>,
    pub d12: port::portd::PD6<mode::io::Input<mode::io::Floating>>,
    pub d13: port::portc::PC7<mode::io::Input<mode::io::Floating>>,
    pub led_rx: port::portb::PB0<mode::io::Input<mode::io::Floating>>,
    pub led_tx: port::portd::PD5<mode::io::Input<mode::io::Floating>>,

    pub ddr: DDR,
}

impl Pins {
    pub fn new(
        dp_portb: atmega32u4::PORTB,
        dp_portc: atmega32u4::PORTC,
        dp_portd: atmega32u4::PORTD,
        dp_porte: atmega32u4::PORTE,
        dp_portf: atmega32u4::PORTF,
    ) -> Pins {
        use atmega32u4_hal::port::PortExt;

        let portb = dp_portb.split();
        let portc = dp_portc.split();
        let portd = dp_portd.split();
        let porte = dp_porte.split();
        let portf = dp_portf.split();

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

            ddr: DDR {
                portb: portb.ddr,
                portc: portc.ddr,
                portd: portd.ddr,
                porte: porte.ddr,
                portf: portf.ddr,
            }
        }
    }
}
