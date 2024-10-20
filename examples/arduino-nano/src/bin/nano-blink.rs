#![no_std]
#![no_main]

use panic_halt as _;

#[niti_eal::entry]
fn main() -> ! {
    let dp = niti_eal::Peripherals::take().unwrap();
    let pins = niti_eal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        led.toggle();
        niti_eal::delay_ms(100);
        led.toggle();
        niti_eal::delay_ms(100);
        led.toggle();
        niti_eal::delay_ms(100);
        led.toggle();
        niti_eal::delay_ms(800);
    }
}
