use esp_idf_hal::ledc::{config::TimerConfig,
    LedcDriver, 
    LedcTimerDriver, 
    Resolution,};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;

fn main() {
    esp_idf_hal::sys::link_patches();

    let perifericos = Peripherals::take().unwrap();

    let tempo = LedcTimerDriver::new(perifericos.ledc.timer0, &TimerConfig::new().frequency(1.kHz().into()).resolution(Resolution::Bits8)).unwrap();
    let mut pin1 = PinDriver::output(perifericos.pins.gpio8).unwrap();
    let mut pin2 = PinDriver::output(perifericos.pins.gpio9).unwrap();
    let mut pin3 = LedcDriver::new(perifericos.ledc.channel0, tempo, perifericos.pins.gpio10).unwrap();


    pin1.set_high().unwrap();
    pin2.set_low().unwrap();
    pin3.set_duty(pin3.get_max_duty()).unwrap();

    loop {}
}