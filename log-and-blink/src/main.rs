use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::*;
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();
    let peripherals = Peripherals::take()?;

    esp_idf_svc::log::EspLogger::initialize_default();
    let mut led = PinDriver::output(peripherals.pins.gpio8)?;

    loop {
        led.set_high()?;
        log::info!("Led encendido!");
        FreeRtos::delay_ms(1000);
        led.set_low()?;
        FreeRtos::delay_ms(1000);
    }
}
