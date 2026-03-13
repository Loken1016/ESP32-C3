use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::adc::oneshot::AdcChannelDriver;
use esp_idf_hal::adc::oneshot::config::AdcChannelConfig;
use esp_idf_hal::adc::oneshot::*;
use esp_idf_hal::adc::attenuation::DB_11;
use esp_idf_hal::delay::FreeRtos;
use embedded_graphics::{
mono_font::{ascii::FONT_6X10, MonoTextStyle},
pixelcolor::BinaryColor,
prelude::*,
text::Text,
};
use esp_idf_hal::{i2c::*, units::*};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

fn main() -> anyhow::Result<()> {
    esp_idf_hal::sys::link_patches();
    let peripherals = Peripherals::take()?;
	let sda = peripherals.pins.gpio8;
	let sck = peripherals.pins.gpio9;

	let config = I2cConfig::new().baudrate(100.kHz().into());
	let i2c = I2cDriver::new(peripherals.i2c0, sda, sck, &config).unwrap();
	let interface = I2CDisplayInterface::new(i2c);
	let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
		.into_buffered_graphics_mode();

	display.init().unwrap();
	display.clear(BinaryColor::Off).unwrap();

	let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

	esp_idf_svc::log::EspLogger::initialize_default();

	let adc = AdcDriver::new(peripherals.adc1)?;
	let config = AdcChannelConfig{
	attenuation: DB_11,
	..Default::default()
};
	let mut x = AdcChannelDriver::new(&adc, peripherals.pins.gpio0,  &config)?;
	let mut y = AdcChannelDriver::new(&adc, peripherals.pins.gpio1, &config)?;


    loop {
	let x_value: u16 = adc.read(&mut x)?;
	let y_value: u16 = adc.read(&mut y)?;
	
	//log::info!("{} {} " ,x_value, y_value);
	
	if (x_value >= 0 && x_value <= 100)
			&& (y_value >= 1400 && y_value <= 1500) {
	log::info!("Derecha");
	display.clear(BinaryColor::Off).unwrap();
	Text::new("Derecha", Point::new(0, 10), style).draw(&mut display).unwrap();
	display.flush().unwrap();
	}
	else if (x_value >= 2200 && x_value <= 2500)
			&& (y_value >= 1400 && y_value <= 1500) {
	log::info!("Izquierda");
	display.clear(BinaryColor::Off).unwrap();
	Text::new("Izquierda", Point::new(0, 10), style).draw(&mut display).unwrap();
	display.flush().unwrap();
	}
	
	if (x_value >= 1400 && x_value <= 1500)
			&& (y_value >= 0 && y_value <= 100) {
	log::info!("Arriba");
	display.clear(BinaryColor::Off).unwrap();
	Text::new("Arriba", Point::new(0, 10), style).draw(&mut display).unwrap();
	display.flush().unwrap();
	}
	else if (x_value >= 1400 && x_value <= 1500)
			&& (y_value >= 2200 && y_value <= 2500) {
	log::info!("Abajo");
	display.clear(BinaryColor::Off).unwrap();
	Text::new("Abajo", Point::new(0, 10), style).draw(&mut display).unwrap();
	display.flush().unwrap();
	}
	
	if (1400..=1500).contains(&x_value)
			&& (1400..=1500).contains(&y_value)
	{
	log::info!("centro");
	display.clear(BinaryColor::Off).unwrap();
	Text::new("Centro", Point::new(0, 10), style).draw(&mut display).unwrap();
	display.flush().unwrap();
	}
	
	FreeRtos::delay_ms(1000);
    }
}
