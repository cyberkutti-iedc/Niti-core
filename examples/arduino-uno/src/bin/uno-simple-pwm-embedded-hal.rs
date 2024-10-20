/*!
 * Example of using simple_pwm to fade a LED in and out on pin d5, but with a twist:
 * the fade function is not aware of `avr-hal` and only uses `embedded-hal` traits.
 */
#![no_std]
#![no_main]

use niti_eal::simple_pwm::*;
use embedded_hal::delay_ms::DelayNs;
use embedded_hal::pwm::SetDutyCycle;
use panic_halt as _;

fn fade(led: &mut impl SetDutyCycle, delay_ms: &mut impl DelayNs) -> ! {
    loop {
        for pct in (0..=100).chain((0..100).rev()) {
            led.set_duty_cycle_percent(pct).unwrap();
            delay_ms.delay_ms(10);
        }
    }
}

#[niti_eal::entry]
fn main() -> ! {
    let dp = niti_eal::Peripherals::take().unwrap();
    let pins = niti_eal::pins!(dp);

    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);

    // Digital pin 5 is connected to a LED and a resistor in series
    let mut pwm_led = pins.d5.into_output().into_pwm(&timer0);
    pwm_led.enable();

    let mut delay_ms = niti_eal::Delay::new();

    fade(&mut pwm_led, &mut delay_ms);
}
