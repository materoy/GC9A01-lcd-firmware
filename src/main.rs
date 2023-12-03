use std::error::Error;
use std::thread::sleep;
use std::time::Duration;
use display_interface_spi::SPIInterface;
use embedded_graphics::image::Image;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::Channel::Pwm0;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use tinybmp::Bmp;


// Pins
const CS_PIN: u8 = 8;
const DC_PIN: u8 = 25;
const RST_PIN: u8 = 27;
const _BL_PIN: u8 = 18;


fn main() -> Result<(), Box<dyn Error>> {
    let mut delay = rppal::hal::Delay::new();
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0)?;

    let gpio = Gpio::new()?;
    let dc = gpio.get(DC_PIN)?.into_output();
    let cs = gpio.get(CS_PIN)?.into_output();
    let rst_pin = gpio.get(RST_PIN)?.into_output();

    let spi_interface :SPIInterface<Spi, OutputPin, OutputPin> = SPIInterface::new(spi, dc, cs);
    //
    let pwm = rppal::pwm::Pwm::new(Pwm0)?;
    pwm.set_period(Duration::from_millis(30))?;
    pwm.enable()?;

    let mut display_driver = gc9a01a::GC9A01A::new(spi_interface, rst_pin, pwm);

    display_driver.reset(&mut delay).unwrap();

    display_driver.set_backlight(550000f64);

    display_driver.initialize(&mut delay).unwrap();

    display_driver.clear(Rgb565::BLUE).unwrap();

    let bmp : Bmp<Rgb565>= Bmp::from_slice(include_bytes!("rust.bmp")).unwrap();

    let image = Image::new(&bmp, Point::new(56, 56));

    image.draw(&mut display_driver).unwrap();

    sleep(Duration::from_millis(2000));
    Ok(())
}
