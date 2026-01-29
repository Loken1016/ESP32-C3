use esp_idf_hal::ledc::{config::TimerConfig,
    LedcDriver, 
    LedcTimerDriver, 
    Resolution};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::delay::FreeRtos;

fn main() {
    esp_idf_hal::sys::link_patches();

    let perifericos = Peripherals::take().unwrap();

    let tempo = LedcTimerDriver::new(perifericos.ledc.timer0, &TimerConfig::new().frequency(440.Hz().into()).resolution(Resolution::Bits8)).unwrap();

    let mut pin3 = LedcDriver::new(perifericos.ledc.channel0, tempo, perifericos.pins.gpio10).unwrap();

    let max_duty = pin3.get_max_duty();

    loop {
    pin3.set_duty(max_duty * 10 / 100).unwrap();
    FreeRtos::delay_ms(2000);
    pin3.set_duty(0).unwrap();
    FreeRtos::delay_ms(2000);   
    }
}
