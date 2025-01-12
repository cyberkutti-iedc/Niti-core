#![no_std]
#![no_main]

use niti_eal::prelude::*;
use panic_halt as _;

use niti_eal::adc;

#[niti_eal::entry]
fn main() -> ! {
    let dp = niti_eal::Peripherals::take().unwrap();
    let pins = niti_eal::pins!(dp);
    let mut serial = niti_eal::default_serial!(dp, pins, 57600);

    let mut adc = niti_eal::Adc::new(dp.ADC, Default::default());

    let (vbg, gnd, tmp) = (
        adc.read_blocking(&adc::channel::Vbg),
        adc.read_blocking(&adc::channel::Gnd),
        adc.read_blocking(&adc::channel::Temperature),
    );
    ufmt::uwriteln!(&mut serial, "Vbandgap: {}", vbg).unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Ground: {}", gnd).unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "Temperature: {}", tmp).unwrap_infallible();

    let a0 = pins.a0.into_analog_input(&mut adc);
    let a1 = pins.a1.into_analog_input(&mut adc);
    let a2 = pins.a2.into_analog_input(&mut adc);
    let a3 = pins.a3.into_analog_input(&mut adc);
    let a4 = pins.a4.into_analog_input(&mut adc);
    let a5 = pins.a5.into_analog_input(&mut adc);

    loop {
        let values = [
            a0.analog_read(&mut adc),
            a1.analog_read(&mut adc),
            a2.analog_read(&mut adc),
            a3.analog_read(&mut adc),
            a4.analog_read(&mut adc),
            a5.analog_read(&mut adc),
        ];

        for (i, v) in values.iter().enumerate() {
            ufmt::uwrite!(&mut serial, "A{}: {} ", i, v).unwrap_infallible();
        }

        // Arduino Nano has two more ADC pins A6 and A7.  Accessing them works a bit different from
        // the other pins as they are not normal IO pins.  The code below shows how it works.
        let (a6, a7) = (
            adc.read_blocking(&adc::channel::ADC6),
            adc.read_blocking(&adc::channel::ADC7),
        );
        ufmt::uwrite!(&mut serial, "A6: {} A7: {}", a6, a7).unwrap_infallible();

        ufmt::uwriteln!(&mut serial, "").unwrap_infallible();
        niti_eal::delay_ms(1000);
    }
}
