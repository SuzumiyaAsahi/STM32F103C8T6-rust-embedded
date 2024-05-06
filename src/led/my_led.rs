use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use stm32f1xx_hal::gpio::{gpioa::Parts, IOPinSpeed, Output, OutputSpeed, Pin};

pub struct Led<const P: char, const N: u8> {
    inner: Pin<P, N, Output>,
}

impl<const P: char, const N: u8> Led<P, N> {
    pub fn light_on(&mut self) {
        self.inner.set_low()
    }

    pub fn light_off(&mut self) {
        self.inner.set_high()
    }

    pub fn led_turn(&mut self) {
        if self.inner.is_set_low() {
            self.inner.set_high();
        } else {
            self.inner.set_low();
        }
    }
}

pub fn led_init(mut gpioa: Parts) -> (Led<'A', 1>, Led<'A', 2>) {
    let mut led1 = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    led1.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    let mut led2 = gpioa.pa2.into_push_pull_output(&mut gpioa.crl);
    led2.set_speed(&mut gpioa.crl, IOPinSpeed::Mhz50);

    led1.set_high();
    led2.set_high();

    (Led { inner: led1 }, Led { inner: led2 })
}
