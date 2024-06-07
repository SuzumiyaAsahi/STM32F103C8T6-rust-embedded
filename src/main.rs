#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m::singleton;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{adc, dma::Half, pac, pac::adc1, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let p = pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.adcclk(6.MHz()).freeze(&mut flash.acr);

    let dma_ch1 = p.DMA1.split().1;

    // Setup ADC
    let mut adc1 = adc::Adc::adc1(p.ADC1, clocks);

    adc1.set_align(adc::Align::Right);
    // 外部触发源，软件触发
    adc1.set_external_trigger(adc1::cr2::EXTSEL_A::Swstart);
    // 设置ADC单次转换
    adc1.set_continuous_mode(true);

    // Setup GPIOA
    let mut gpioa = p.GPIOA.split();

    // Configure pa0 as an analog input
    let ch0 = gpioa.pa0.into_analog(&mut gpioa.crl);

    let adc_dma = adc1.with_dma(ch0, dma_ch1);
    let buf = singleton!(: [[u16; 8]; 2] = [[0; 8]; 2]).unwrap();

    let mut circ_buffer = adc_dma.circ_read(buf);
    rprintln!("hello");
    loop {
        // 真是整不动了，暂时只能转换一次。
        while circ_buffer.readable_half().unwrap() != Half::First {}
        let first_half = circ_buffer.peek(|half, _| *half).unwrap();
        rprintln!("this is First");
        first_half.iter().for_each(|x| rprintln!("{}", x));
    }
}
