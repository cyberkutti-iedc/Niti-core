#![no_std]
#![no_main]

use panic_halt as _;

#[niti_hal::entry]
fn main() -> ! {
    let dp = niti_hal::Peripherals::take().unwrap();
    let pins = niti_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        led.toggle();
        niti_hal::delay_ms(100);
        led.toggle();
        niti_hal::delay_ms(100);
        led.toggle();
        niti_hal::delay_ms(100);
        led.toggle();
        niti_hal::delay_ms(800);
    }
}
