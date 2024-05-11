#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::{cell::RefCell, ops::Add};
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use pac::interrupt;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{
    gpio::{Edge, ExtiPin, Input, Pin, PullUp},
    pac,
    prelude::*,
};

type Dectector = Mutex<RefCell<Option<Pin<'B', 14, Input<PullUp>>>>>;

type Counter = Mutex<RefCell<Option<u32>>>;

#[allow(non_upper_case_globals)]
static _jack: Dectector = Mutex::new(RefCell::new(None));

#[allow(non_upper_case_globals)]
static _rose: Counter = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut dp = pac::Peripherals::take().unwrap();

    let mut gpiob = dp.GPIOB.split();
    let mut afio = dp.AFIO.constrain();
    let mut infraed_sensor = gpiob.pb14.into_pull_up_input(&mut gpiob.crh);
    infraed_sensor.make_interrupt_source(&mut afio);

    infraed_sensor.trigger_on_edge(&mut dp.EXTI, Edge::Falling);

    infraed_sensor.enable_interrupt(&mut dp.EXTI);

    rprintln!("Hello, world!");

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI15_10);
    }

    cortex_m::interrupt::free(|cs| {
        _jack.borrow(cs).replace(Some(infraed_sensor));
        _rose.borrow(cs).replace(Some(0));
    });

    #[allow(clippy::empty_loop)]
    loop {}
}

#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        let mut infrared_detection = _jack.borrow(cs).borrow_mut();
        if infrared_detection.as_mut().unwrap().check_interrupt() {
            let mut rose = _rose.borrow(cs).borrow_mut();
            let rose = rose.as_mut().unwrap();
            *rose = rose.add(1);
            rprintln!("the rose is {}", rose);
            infrared_detection
                .as_mut()
                .unwrap()
                .clear_interrupt_pending_bit()
        }
    })
}
