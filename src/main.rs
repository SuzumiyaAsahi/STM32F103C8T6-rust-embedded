#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32f1xx_hal::{gpio::OutputSpeed, pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split();

    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);
    led.set_speed(&mut gpioa.crl, stm32f1xx_hal::gpio::IOPinSpeed::Mhz50);

    let mut timer = Timer::syst(cp.SYST, &clocks).delay();

    loop {
        led.set_low();
        timer.delay_ms(500_u16);
        led.set_high();
        timer.delay_ms(500_u16);
    }
}
