#![no_std]
#![no_main]

use defmt::*;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_time::Timer;
use panic_probe as _;

use embassy_stm32::{self as _, Config};

// ADC
use embassy_stm32::adc::{Adc, Resolution};

// PWM
use embassy_stm32::gpio::OutputType;
use embassy_stm32::peripherals::{ADC1, TIM2};
use embassy_stm32::time::hz;
use embassy_stm32::timer::{Ch3, simple_pwm::PwmPin, simple_pwm::SimplePwm};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Config::default());
    info!("Smile & Wave starting");

    let mut adc = Adc::new(p.ADC1);
    adc.set_resolution(Resolution::BITS12);
    let max_adc: u32 = (1 << 12) - 1;

    let mut joy_x = p.PA0;

    let servo_pin: PwmPin<'_, TIM2, Ch3> = PwmPin::new(p.PB10, OutputType::PushPull);

    let mut servo_pwm = SimplePwm::new(
        p.TIM2,
        None,
        None,
        Some(servo_pin),
        None,
        hz(50),
        Default::default(),
    );
    let max_duty: u16 = servo_pwm.max_duty_cycle();

    let mut servo_ch = servo_pwm.ch3();
    servo_ch.enable();

    // Servo pulse range
    const SERVO_PERIOD_S: f32 = 1.0 / 50.0; // 20 ms
    const SERVO_MIN_PW_S: f32 = 0.2e-3; // 1.0 ms
    const SERVO_MAX_PW_S: f32 = 3.0e-3; // 2.0 ms

    loop {
        // 1. Read joystick
        let raw: u16 = adc.blocking_read(&mut joy_x);
        let t = (raw as f32 / max_adc as f32).clamp(0.0, 1.0);

        // 2. Map joystick -> pulse width
        let pulse_s = SERVO_MIN_PW_S + t * (SERVO_MAX_PW_S - SERVO_MIN_PW_S);

        // 3. Pulse width -> duty
        let duty_f = (pulse_s / SERVO_PERIOD_S) * max_duty as f32;
        let duty_u16: u16 = duty_f.clamp(0.0, max_duty as f32) as u16;

        servo_ch.set_duty_cycle(duty_u16);

        info!("adc={} t={:?} duty={}", raw, t, duty_u16);

        Timer::after_millis(20).await;
    }
}
