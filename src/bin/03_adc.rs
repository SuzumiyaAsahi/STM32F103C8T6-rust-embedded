#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32f1xx_hal::adc;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();

    // 选择时钟6分频
    let clocks = rcc.cfgr.adcclk(6.MHz()).freeze(&mut flash.acr);

    let mut adc1 = adc::Adc::adc1(dp.ADC1, clocks);

    let mut gpioa = dp.GPIOA.split();

    let mut ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);

    loop {
        let data: u16 = adc1.read(&mut ch0).unwrap();
        rprintln!("adc1: {}", data);
    }
}
