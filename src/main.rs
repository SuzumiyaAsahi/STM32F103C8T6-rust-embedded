#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::singleton;
use cortex_m_rt::entry;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32f1xx_hal::dma::Half;
use stm32f1xx_hal::{adc, pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.adcclk(6.MHz()).freeze(&mut flash.acr);

    let dma_ch1 = p.DMA1.split().1;

    // Setup ADC
    let adc1 = adc::Adc::adc1(p.ADC1, clocks);

    // Setup GPIOA
    let mut gpioa = p.GPIOA.split();

    // Configure pa0 as an analog input
    let adc_ch5 = gpioa.pa5.into_analog(&mut gpioa.crl);

    let adc_dma = adc1.with_dma(adc_ch5, dma_ch1);
    let buf = singleton!(: [[u16; 8]; 2] = [[0; 8]; 2]).unwrap();

    let mut circ_buffer = adc_dma.circ_read(buf);

    while circ_buffer.readable_half().unwrap() != Half::First {}

    let _first_half = circ_buffer.peek(|half, _| *half).unwrap();

    while circ_buffer.readable_half().unwrap() != Half::Second {}

    let _second_half = circ_buffer.peek(|half, _| *half).unwrap();

    let (_buf, adc_dma) = circ_buffer.stop();
    let (_adc1, _adc_ch0, _dma_ch1) = adc_dma.split();
    
    rprintln!("hello");

    loop {}
}
