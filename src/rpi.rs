use std::time::Duration;
use display_interface_spi::SPIInterface;
use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::RgbColor;
use gc9a01a::GC9A01A;
use rppal::gpio::{Gpio, OutputPin};
use rppal::pwm::Channel::Pwm0;
use rppal::pwm::Pwm;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

// Pins
const CS_PIN: u8 = 8;
const DC_PIN: u8 = 25;
const RST_PIN: u8 = 27;
const _BL_PIN: u8 = 18;


fn get_rpi02w_display_driver<'a>() -> GC9A01A<SPIInterface<Spi, OutputPin, OutputPin>, OutputPin, Pwm> {
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0).unwrap();

    let gpio = Gpio::new().unwrap();
    let dc = gpio.get(DC_PIN).unwrap().into_output();
    let cs = gpio.get(CS_PIN).unwrap().into_output();
    let rst_pin = gpio.get(RST_PIN).unwrap().into_output();
    let spi_interface: SPIInterface<Spi, OutputPin, OutputPin> = SPIInterface::new(spi, dc, cs);
    let pwm = Pwm::new(Pwm0).unwrap();
    pwm.set_period(Duration::from_millis(30)).unwrap();
    pwm.enable().unwrap();
    let display_driver = GC9A01A::new(spi_interface, rst_pin, pwm);
    display_driver
}
pub fn rpi() -> GC9A01A<SPIInterface<Spi, OutputPin, OutputPin>, OutputPin, Pwm> {
    let mut delay = rppal::hal::Delay::new();
    let mut display_driver = get_rpi02w_display_driver();
    display_driver.reset(&mut delay).unwrap();

    display_driver.set_backlight(550000f64);

    display_driver.initialize(&mut delay).unwrap();

    display_driver.clear(Rgb565::BLACK).unwrap();

    // draw(&mut display_driver).unwrap();
    display_driver
}