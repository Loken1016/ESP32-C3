use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use esp_idf_hal::{i2c::*, peripherals::Peripherals, units::*};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() {
    esp_idf_hal::sys::link_patches();

    let perifericos = Peripherals::take().unwrap();
    let sda = perifericos.pins.gpio8;
    let sck = perifericos.pins.gpio9;

    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(perifericos.i2c0, sda, sck, &config).unwrap();
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    display.clear(BinaryColor::Off).unwrap();

    let estilo = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    Text::new("Hola desde Esp32", Point::new(0, 10), estilo)
        .draw(&mut display)
        .unwrap();
    display.flush().unwrap();

    loop {}
}
