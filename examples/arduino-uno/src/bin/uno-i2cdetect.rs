/*!
 * Detect all devices connected on the I2C/TWI bus.  Useful if you can't figure out the address of
 * an I2C device.
 *
 * This example will check all possible addresses on the I2C bus for whether a device responds to
 * them.  It will output a table of the results.  This check is done twice, once for reading and
 * once for writing, as some devices only respond to one of the two operations.
 *
 * ATTENTION: Randomly reading from and writing to devices can lead to unexpected results.  Some
 * devices do not cope well with this.  Use with care!
 *
 * Connections
 * -----------
 *  - `A4`: I2C SDA signal
 *  - `A5`: I2C SCL signal
 */
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
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").unwrap_infallible();
    i2c.i2cdetect(&mut serial, niti_eal::i2c::Direction::Write)
        .unwrap_infallible();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").unwrap_infallible();
    i2c.i2cdetect(&mut serial, niti_eal::i2c::Direction::Read)
        .unwrap_infallible();

    loop {
        niti_eal::delay_ms(1000);
    }
}
