// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use stm32f1xx_hal::{
    gpio::{gpiob::Parts, Input, Pin, PullUp},
    prelude::*,
    timer::delay::SysDelay,
};

pub struct Key<const P: char, const N: u8> {
    inner: Pin<P, N, Input<PullUp>>,
}

impl<const P: char, const N: u8> Key<P, N> {
    pub fn is_entered(&mut self, timer: &mut SysDelay) -> bool {
        if self.inner.is_low() {
            timer.delay_ms(20_u16);
            while self.inner.is_low() {}
            timer.delay_ms(20_u16);
            return true;
        }
        false
    }
}

pub fn key_init(mut gpiob: Parts) -> (Key<'B', 0>, Key<'B', 11>) {
    let key1 = gpiob.pb0.into_pull_up_input(&mut gpiob.crl);

    let key2 = gpiob.pb11.into_pull_up_input(&mut gpiob.crh);

    (Key { inner: key1 }, Key { inner: key2 })
}
