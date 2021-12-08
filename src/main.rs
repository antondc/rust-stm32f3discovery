#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use panic_itm;

use cortex_m_rt::entry;
use stm32f3_discovery::{leds::Direction, leds::Leds, stm32f3xx_hal, switch_hal};
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::{delay::Delay, hal::blocking::delay::DelayMs, pac};
use switch_hal::OutputSwitch;

#[entry]
fn main() -> ! {
    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_periphs.RCC.constrain();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    let mut gpioe = device_periphs.GPIOE.split(&mut reset_and_clock_control.ahb);
    let mut leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    let miliseconds: u16 = 300;

    loop {
        leds.for_direction(Direction::North).on().ok();
        delay.delay_ms(miliseconds);

        leds.for_direction(Direction::North).off().ok();
        delay.delay_ms(miliseconds);
    }
}
