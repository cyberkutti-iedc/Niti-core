/*!
 * Demonstration of writing to and reading from the eeprom.
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

    let mut ep = niti_eal::Eeprom::new(dp.EEPROM);
    let ep_capacity = ep.capacity();
    ufmt::uwriteln!(&mut serial, "eeprom capacity is:{}\r", ep_capacity).unwrap_infallible();

    // KNOWN ISSUE: Avoid to read entire eeprom capacity at once
    // See: https://github.com/Rahix/avr-hal/issues/410
    let mut data = [0_u8; 10];

    let _start_address: u16 = 0;

    if ep.read(0, &mut data).is_err() {
        ufmt::uwriteln!(&mut serial, "read eeprom fail:\r").unwrap_infallible();
        loop {}
    }

    ufmt::uwriteln!(&mut serial, "Got:\r").unwrap_infallible();
    for i in data {
        ufmt::uwriteln!(&mut serial, "{}", i).unwrap_infallible();
    }

    let _ = ep.erase(0, niti_eal::Eeprom::CAPACITY);

    loop {}
}
