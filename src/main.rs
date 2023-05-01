#![no_std]
#![no_main]

// pick a panicking behavior
// use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::delay::Delay;

use stm32f3_discovery::switch_hal::{IntoSwitch, OutputSwitch, ToggleableOutputSwitch};

#[entry]
fn main() -> ! {

    let device_periphs = pac::Peripherals::take().unwrap();
    let mut reset_control_clock = device_periphs.RCC.constrain();
    let mut gpioe = device_periphs.GPIOE.split(&mut reset_control_clock.ahb);

    let mut led =
            gpioe
            .pe13
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
            .into_active_high_switch();

    led.off().unwrap();

    let core_periphs = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_periphs.FLASH.constrain();
    let clocks = reset_control_clock.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_periphs.SYST, clocks);

    loop {
        led.toggle().unwrap();
        delay.delay_ms(1000u16);
    }
}
