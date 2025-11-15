#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::Pull;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let mut buttonS1 = ExtiInput::new(p.PB4, p.EXTI4, Pull::Up);
    let mut buttonS2 = ExtiInput::new(p.PB5, p.EXTI5,Pull::Up);
    let mut buttonS3 = ExtiInput::new(p.PB3, p.EXTI3,Pull::Up);
    let mut buttonS4 = ExtiInput::new(p.PC8, p.EXTI8, Pull::Up);

    loop {
        if buttonS1.is_low() {
            defmt::info!("SQUAD CAL RETARDED");
            buttonS1.wait_for_high().await;
        }

        if buttonS2.is_low() {
            defmt::info!("SQUAD CAL RETARDED LMAO");
            buttonS2.wait_for_high().await;
        }

        if buttonS3.is_low() {
            defmt::info!("SQUAD CAL RETARDED BRUH");
            buttonS3.wait_for_high().await;
        }

        if buttonS4.is_low() {
            defmt::info!("SQUAD CAL RETARDED YOOOO");
            buttonS4.wait_for_high().await;
        }

        // Wait for 100 milliseconds before checking again
        Timer::after(embassy_time::Duration::from_millis(100)).await;
    }
}
