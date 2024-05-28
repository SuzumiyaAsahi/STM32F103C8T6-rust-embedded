#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use pac::interrupt;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32f1xx_hal::{
    gpio::{Edge, ExtiPin, IOPinSpeed, Input, Output, OutputSpeed, Pin, PullUp},
    pac,
    prelude::*,
};

type Dectector = Mutex<RefCell<Option<Pin<'A', 8, Input<PullUp>>>>>;
type LED = Mutex<RefCell<Option<Pin<'B', 1, Output>>>>;

#[allow(non_upper_case_globals)]
static _jack: Dectector = Mutex::new(RefCell::new(None));

#[allow(non_upper_case_globals)]
static _rose: LED = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut dp = pac::Peripherals::take().unwrap();

    let mut gpioa = dp.GPIOA.split();
    let mut gpiob = dp.GPIOB.split();

    let mut led_pb1 = gpiob.pb1.into_push_pull_output(&mut gpiob.crl);
    led_pb1.set_speed(&mut gpiob.crl, IOPinSpeed::Mhz50);
    led_pb1.set_high();

    let mut afio = dp.AFIO.constrain();
    let mut infraed_sensor = gpioa.pa8.into_pull_up_input(&mut gpioa.crh);
    infraed_sensor.make_interrupt_source(&mut afio);

    infraed_sensor.trigger_on_edge(&mut dp.EXTI, Edge::Falling);

    infraed_sensor.enable_interrupt(&mut dp.EXTI);

    rprintln!("Hello, world!");

    unsafe {
        pac::NVIC::unmask(pac::Interrupt::EXTI9_5);
    }

    cortex_m::interrupt::free(|cs| {
        _jack.borrow(cs).replace(Some(infraed_sensor));
        _rose.borrow(cs).replace(Some(led_pb1));
    });

    #[allow(clippy::empty_loop)]
    loop {}
}

#[interrupt]
fn EXTI9_5() {
    cortex_m::interrupt::free(|cs| {
        let mut infrared_detection = _jack.borrow(cs).borrow_mut();
        if infrared_detection.as_mut().unwrap().check_interrupt() {
            rprint!("low power detected");
            let mut rose = _rose.borrow(cs).borrow_mut();
            let rose = rose.as_mut().unwrap();
            rose.toggle();
            infrared_detection
                .as_mut()
                .unwrap()
                .clear_interrupt_pending_bit()
        }
    })
}
