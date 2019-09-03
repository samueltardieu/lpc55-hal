#![no_main]
#![no_std]

extern crate panic_semihosting;
use cortex_m_rt::entry;

use lpc55s6x_hal as hal;
use hal::traits::*;

#[entry]
fn main() -> ! {

    let peripherals = hal::Peripherals::take().unwrap();

    let mut syscon = peripherals.SYSCON.split();

    let gpio = peripherals.GPIO.enable(&mut syscon.handle);
    let iocon = peripherals.IOCON.split();
    // UM kind of says it's not enabled, but it actually is
    iocon.handle.enable(&mut syscon.handle);

    // R = pio1_6
    // G = pio1_7
    // B = pio1_4
    //
    // on = low, off = high

    let mut red = iocon.pins.pio1_6
        .into_gpio_pin(&gpio)
        .into_output(hal::gpio::Level::High);  // start turned off

	let clock = syscon.fro_1mhz_utick_clock.enable(&mut syscon.handle);
	let delay = hal::clock::Ticks { value: 500_000, clock: &clock }; // 500 ms = 0.5 s

    let mut utick = peripherals.UTICK.enable(&mut syscon.handle);
	let mut sleep = hal::sleep::Busy::prepare(&mut utick);

    // use this order to check whether LED initially flashes up
    loop {
        // this is to workaround the v1/v2 digital pin
        // situation, until Vadim's v3 lands

		sleep.sleep(delay);
        red.set_low().unwrap();

		sleep.sleep(delay);
        red.set_high().unwrap();
    }
}
