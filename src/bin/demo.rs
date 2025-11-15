#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::gpio::OutputType;
use embassy_stm32::peripherals::TIM1;
use embassy_stm32::peripherals::TIM2;
use embassy_stm32::peripherals::TIM3;
use embassy_stm32::time::khz;
use embassy_stm32::timer::Ch1;

use embassy_stm32::timer::Ch2;
use embassy_stm32::timer::Ch3;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::{self as _, Config};
use embassy_time::Timer;
use panic_probe as _;


use embassy_stm32::bind_interrupts;
use embassy_stm32::i2c::{self, I2c};



#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let config = Config::default();
    let p = embassy_stm32::init(Default::default());
    let led_pwm_pin: PwmPin<'_, TIM2, Ch2> = PwmPin::new(p.PB3, OutputType::PushPull);
    let mut pwm = SimplePwm::new(
        p.TIM2,
        None,
        Some(led_pwm_pin),
        None,
        None,
        khz(1),
        Default::default(),
    );


    let led2_pwm_pin: PwmPin<'_, TIM3, Ch1> = PwmPin::new(p.PB4, OutputType::PushPull);
    let mut pwm2 = SimplePwm::new(
        p.TIM3,
        Some(led2_pwm_pin),
        None,
        None,
        None,
        khz(1),
        Default::default(),
    );




    let mut ch2 = pwm.ch2(); // Get handle for channel 2
    ch2.enable();
    let mut ch1 = pwm2.ch1(); // Get handle for channel 1
    ch1.enable();
    
    ch2.set_duty_cycle_percent(99);
    ch1.set_duty_cycle_percent(99);



    let mut bmp = Bmp280::new_async(i2c, Delay);
    
    if let Err(e) = bmp.init().await {
        defmt::error!("Failed to initialize BMP280: {:?}", e);
        loop {}
    }


    
    loop {
        for i in 99..2 {
            ch2.set_duty_cycle_percent(i);
            Timer::after_millis(100).await;
        }
        for i in 2..99 {
            ch2.set_duty_cycle_percent(i);
            Timer::after_millis(100).await;
        }

        for i in 99..2 {
            ch1.set_duty_cycle_percent(i);
            Timer::after_millis(100).await;
        }
        for i in 2..99 {
            ch1.set_duty_cycle_percent(i);
            Timer::after_millis(100).await;
        }




        /*  Timer::after_secs(1).await;
        ch2.set_duty_cycle_percent(2);
        ch1.set_duty_cycle_percent(99);
        Timer::after_secs(1).await;
        ch2.set_duty_cycle_percent(99);
        ch1.set_duty_cycle_percent(2);*/
    }
}