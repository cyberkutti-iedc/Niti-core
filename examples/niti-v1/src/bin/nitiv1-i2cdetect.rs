#![no_std]
#![no_main]

use niti_hal::prelude::*;
use panic_halt as _;

#[niti_hal::entry]
fn main() -> ! {
    let dp = niti_hal::Peripherals::take().unwrap();
    let pins = niti_hal::pins!(dp);
    let mut serial = niti_hal::default_serial!(dp, pins, 57600);

    let mut i2c = niti_hal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").unwrap_infallible();
    i2c.i2cdetect(&mut serial, niti_hal::i2c::Direction::Write)
        .unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").unwrap_infallible();
    i2c.i2cdetect(&mut serial, niti_hal::i2c::Direction::Read)
        .unwrap_infallible();

    loop {}
}
