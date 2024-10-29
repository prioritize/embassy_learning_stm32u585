#![no_std]
#![no_main]

mod fmt;

#[cfg(not(feature = "defmt"))]
use panic_halt as _;
#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals::{PH6, PH7};
use embassy_time::{Duration, Timer};
use fmt::info;

#[embassy_executor::task]
async fn user_led_0_blinker(mut led: Output<'static, PH6>, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
        info!("user_led_0_blinker");
    }
}
#[embassy_executor::task]
async fn user_led_1_blinker(mut led: Output<'static, PH7>, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
        info!("user_led_1_blinker");
    }
}
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut user_led_0 = Output::new(p.PH6, Level::High, Speed::Low);
    let mut user_led_1 = Output::new(p.PH7, Level::High, Speed::Low);
    let interval = Duration::from_millis(500);
    spawner.spawn(user_led_0_blinker(user_led_0, interval));
    spawner.spawn(user_led_1_blinker(user_led_1, interval));

    // loop {
    //     info!("Hello, World!");
    //     user_led_0.set_high();
    //     user_led_1.set_high();
    //     Timer::after(Duration::from_millis(500)).await;
    //     user_led_0.set_low();
    //     user_led_1.set_low();
    //     Timer::after(Duration::from_millis(500)).await;
    // }
}
