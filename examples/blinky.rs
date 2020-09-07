//! Blinks an LED

#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;
// #[macro_use(block)]
// extern crate nb;

use crate::hal::delay::Delay;
use crate::hal::prelude::*;
use crate::rt::entry;
use crate::rt::ExceptionFrame;

use crate::sh::hio;
use core::fmt::Write;

#[entry]
fn main() -> ! {
    //let mut hstdout = hio::hstdout().unwrap();

    //writeln!(hstdout, "Hello, world!").unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain(); // .constrain();
    let mut rcc = dp.RCC.constrain();
    let mut pwr = dp.PWR.constrain(&mut rcc.apb1r1);

    // Try a different clock configuration
    // let clocks = rcc.cfgr.freeze(&mut flash.acr, &mut pwr);
    // let clocks = rcc.cfgr.hclk(32.mhz()).freeze(&mut flash.acr, &mut pwr);
    let clocks = rcc
        .cfgr
        .sysclk(80.mhz())
        .pclk1(80.mhz())
        .pclk2(80.mhz())
        .freeze(&mut flash.acr, &mut pwr);

    // let mut gpioc = dp.GPIOC.split(&mut rcc.ahb2);
    // let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.afrh);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);
    let mut led = gpioa
        .pa5
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    let mut pa6 = gpioa
        .pa6
        .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);

    let mut timer = Delay::new(cp.SYST, clocks);
    let data = [0b10101010u8, 0b01010101u8, 0b10000000u8, 0b10001000u8];
    loop {
        for (i, d) in data.iter().enumerate() {
            if i % 2 == 0 {
                pa6.set_low();
            } else {
                pa6.set_high();
            }
            let mut d = *d;
            for j in 0..8 {
                if d & (0x1 as u8) == 0x1 {
                    led.set_high();
                } else {
                    led.set_low();
                }
                d = d >> 1;
            }
        }
    }

    loop {
        // block!(timer.wait()).unwrap();
        // timer.delay_ms(200 as u32);
        led.set_high().unwrap();
        pa6.set_high().unwrap();
        led.set_low().unwrap();
        pa6.set_low().unwrap();
        led.set_high().unwrap();
        pa6.set_high().unwrap();
        led.set_low().unwrap();
        pa6.set_low().unwrap();
        led.set_high().unwrap();
        pa6.set_high().unwrap();
        led.set_low().unwrap();
        pa6.set_low().unwrap();
        led.set_high().unwrap();
        pa6.set_high().unwrap();
        led.set_low().unwrap();
        pa6.set_low().unwrap();
        // led.set_high().unwrap();
        // led.set_low().unwrap();
        // led.set_high().unwrap();
        // led.set_low().unwrap();
        // led.set_high().unwrap();
        // led.set_low().unwrap();

        // pa6.set_high().unwrap();
        // block!(timer.wait()).unwrap();
        // timer.delay_ms(200 as u32);
        // pa6.set_low().unwrap();
        // led.set_high();
        // // block!(timer.wait()).unwrap();
        // // timer.delay_ms(200 as u32);
        // led.set_low();
        // led.set_high();
        // // block!(timer.wait()).unwrap();
        // // timer.delay_ms(200 as u32);
        // led.set_low();
        // led.set_high();
        // // block!(timer.wait()).unwrap();
        // // timer.delay_ms(200 as u32);
        // led.set_low();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
