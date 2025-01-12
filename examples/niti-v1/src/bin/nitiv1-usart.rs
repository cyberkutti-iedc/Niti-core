#![no_std]
#![no_main]

use niti_hal::prelude::*;
use panic_halt as _;

use embedded_hal_v0::serial::Read;

#[niti_hal::entry]
fn main() -> ! {
    let dp = niti_hal::Peripherals::take().unwrap();
    let pins = niti_hal::pins!(dp);
    let mut serial = niti_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap_infallible();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).unwrap_infallible();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap_infallible();
    }
}
