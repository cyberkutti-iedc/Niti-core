#![no_std]
#![no_main]

use niti_eal::prelude::*;
use panic_halt as _;

#[niti_eal::entry]
fn main() -> ! {
    let dp = niti_eal::Peripherals::take().unwrap();
    let pins = niti_eal::pins!(dp);
    let mut serial = niti_eal::default_serial!(dp, pins, 57600);

    let mut i2c = niti_eal::I2c::new(
        dp.TWI,
        pins.d20.into_pull_up_input(),
        pins.d21.into_pull_up_input(),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").unwrap_infallible();
    i2c.i2cdetect(&mut serial, niti_eal::i2c::Direction::Write)
        .unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").unwrap_infallible();
    i2c.i2cdetect(&mut serial, niti_eal::i2c::Direction::Read)
        .unwrap_infallible();

    loop {}
}
