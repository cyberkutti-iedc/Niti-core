#![no_std]
#![no_main]

use panic_halt as _;

#[niti_eal::entry]
fn main() -> ! {
    let dp = niti_eal::Peripherals::take().unwrap();
    let pins = niti_eal::pins!(dp);

    let mut led = pins.d13.into_output().downgrade();

    loop {
        led.toggle();
        niti_eal::delay_ms(1000);
    }
}
