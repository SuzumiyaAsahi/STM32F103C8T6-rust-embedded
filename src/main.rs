#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use key::my_key;
use led::my_led;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[path = "./key/mod.rs"]
mod key;

#[path = "./led/mod.rs"]
mod led;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut timer = Timer::syst(cp.SYST, &clocks).delay();

    let (mut led1, mut led2) = my_led::led_init(dp.GPIOA.split());

    let (mut key1, mut key2) = my_key::key_init(dp.GPIOB.split());

    rprintln!("Hello, world!");
    loop {
        if key1.is_entered(&mut timer) {
            led1.led_turn();
        }

        if key2.is_entered(&mut timer) {
            led2.led_turn();
        }
    }
}
