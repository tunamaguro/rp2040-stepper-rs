//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::{digital::OutputPin, pwm::SetDutyCycle};
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico::{self as bsp, pac::PWM};
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac, pwm,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut stepper_dir = pins.gpio15.into_push_pull_output();
    stepper_dir.set_high().unwrap();

    // let pwm_slices = pwm::Slices::new(pac.PWM, &mut pac.RESETS);
    // let mut pwm7 = pwm_slices.pwm7;
    // pwm7.set_ph_correct();
    // pwm7.enable();

    // let mut stepper_pwm = pwm7.channel_a;
    // stepper_pwm.output_to(pins.gpio14);

    let mut stepper_pin = pins.gpio14.into_push_pull_output();

    loop {
        stepper_pin.set_high().unwrap();
        delay.delay_ms(100);
        stepper_pin.set_low().unwrap();
        delay.delay_ms(100);
    }
}

// End of file
