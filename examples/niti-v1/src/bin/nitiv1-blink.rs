#![no_std]
#![no_main]

use panic_halt as _;

#[niti_hal::entry]
fn main() -> ! {
    let dp = niti_hal::Peripherals::take().unwrap();
    let pins = niti_hal::pins!(dp);

    let mut led = pins.d13.into_output().downgrade();

    loop {
        led.toggle();
        niti_hal::delay_ms(1000);
    }
}
