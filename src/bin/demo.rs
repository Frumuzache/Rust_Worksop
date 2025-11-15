#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::Input;
use embassy_stm32::gpio::Pull;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let buttonS1 = Input::new(p.PB4, Pull::Up);
    let buttonS2 = Input::new(p.PB5, Pull::Up);
    let buttonS3 = Input::new(p.PB3, Pull::Up);
    let buttonS4 = Input::new(p.PC8, Pull::Up);

    loop {
        if buttonS1.is_low() {
            defmt::info!("SQUAD CAL RETARDED");
        }

        if buttonS2.is_low() {
            defmt::info!("SQUAD CAL RETARDED LMAO");
        }

        if buttonS3.is_low() {
            defmt::info!("SQUAD CAL RETARDED BRUH");
        }

        if buttonS4.is_low() {
            defmt::info!("SQUAD CAL RETARDED YOOOO");
        }

        // Wait for 100 milliseconds before checking again
        Timer::after(embassy_time::Duration::from_millis(100)).await;
    }
}
