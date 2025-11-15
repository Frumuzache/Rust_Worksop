#![no_std]
#![no_main]


use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::{self as _, Config};
use defmt_rtt as _;
use embassy_time::Timer;
use panic_probe as _;
use embassy_stm32::time::khz;
use embedded_workshop_skeleton::music::{Note, OCTAVE};



/// Beats per minute.
const TEMPO: u64 = 100;
/// A whole note duration in milliseconds.
const WHOLE_NOTE: u64 = 4 * (60_000 / TEMPO);
/// The microcontroller clock frequency
const CLOCK_FREQ: u64 = 150_000_000;
/// PWM clock divider
const PWM_DIV: u64 = 64;


use embassy_stm32::time::hz;
use embassy_stm32::timer::simple_pwm::{PwmPin, SimplePwm};
use embassy_stm32::peripherals::TIM2;
use embassy_stm32::timer::{Ch1, Ch2};
use embassy_stm32::gpio::OutputType;

// The original song is about 114 BPM
// You might want to change your TEMPO constant:
// const TEMPO: u64 = 114;



#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    
    let config = Config::default();
    let p = embassy_stm32::init(config);

    let buzzer: PwmPin<'_, TIM2, Ch2> = PwmPin::new(p.PB3, OutputType::PushPull);


    let mut pwm = SimplePwm::new(
    p.TIM2,              // Timer 2 peripheral
    None,   
    Some(buzzer),                // Channel 2 not used
    None,                // Channel 3 not used
    None,                // Channel 4 not used
    khz(1),              // PWM frequency = 1 kHz
    Default::default(),  // Default configuration
    );



    info!("Playing melody...");
    pwm.ch2().enable();
    
    
loop {
    for (note, duration) in SONG.iter() {
        
        if let Some(n) = note {
            pwm.set_frequency(hz((*n as u32)));
            pwm.ch2().set_duty_cycle_percent(50);
        } 
        
        else {
            pwm.ch2().set_duty_cycle(0); // Silence
        }
        
        let note_duration = WHOLE_NOTE / (*duration as u64);
        Timer::after(embassy_time::Duration::from_millis(note_duration)).await;
    }
}



}
