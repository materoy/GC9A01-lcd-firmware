use std::time::Duration;
use display_interface_spi::SPIInterface;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{
    Circle, CornerRadii, PrimitiveStyleBuilder, Rectangle, RoundedRectangle, StyledDrawable,
};
use embedded_graphics::text::Text;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
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

const DISPLAY_SIZE: u32 = 240;

fn mcu_host() -> bool {
    match Gpio::new() {
        Ok(_) => true,
        Err(_) => false
    }
}

fn get_rpi02w_display_driver<'a>() -> GC9A01A<SPIInterface<Spi, OutputPin, OutputPin>, OutputPin, Pwm> {
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0).unwrap();

    let gpio = Gpio::new().unwrap();
    let dc = gpio.get(DC_PIN).unwrap().into_output();
    let cs = gpio.get(CS_PIN).unwrap().into_output();
    let rst_pin = gpio.get(RST_PIN).unwrap().into_output();
    let spi_interface: SPIInterface<Spi, OutputPin, OutputPin> = SPIInterface::new(spi, dc, cs);
    let pwm = rppal::pwm::Pwm::new(Pwm0).unwrap();
    pwm.set_period(Duration::from_millis(30)).unwrap();
    pwm.enable().unwrap();
    let display_driver = GC9A01A::new(spi_interface, rst_pin, pwm);
    display_driver
}

fn main() -> ! {
    let mut delay = rppal::hal::Delay::new();

    match mcu_host() {
        true => {}
        false => {}
    }

    match Gpio::new() {
        Ok(_) => {
            let mut display_driver = get_rpi02w_display_driver();
            display_driver.reset(&mut delay).unwrap();

            display_driver.set_backlight(550000f64);

            display_driver.initialize(&mut delay).unwrap();

            display_driver.clear(Rgb565::BLACK).unwrap();

            draw(&mut display_driver).unwrap();

        }
        Err(_) => {
            let mut simulator_display = SimulatorDisplay::<Rgb565>::new(Size::new(240, 240));
            let output_settings = OutputSettingsBuilder::new()
                .build();
            draw(&mut simulator_display).unwrap();

            Window::new("1.28 in display", &output_settings).show_static(&simulator_display);
        }
    }


    loop {}
}

fn draw<D: DrawTarget<Color=Rgb565>>(display_driver: &mut D) -> Result<(), D::Error> {
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(4)
        .stroke_color(Rgb565::GREEN)
        .build();

    // Outline Circle
    Circle::new(Point::new(0, 0), DISPLAY_SIZE)
        .draw_styled(&style, display_driver)?;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(2)
        .stroke_color(Rgb565::GREEN)
        .build();

    let rectangle_width = 100;
    let rec_x = (DISPLAY_SIZE - rectangle_width) / 2;
    RoundedRectangle::new(
        Rectangle::new(
            Point::new(rec_x as i32, 20),
            Size::new_equal(rectangle_width),
        ),
        CornerRadii::new(Size::new_equal(4)),
    )
        .draw_styled(&style, display_driver)?;


    let text = "Hello Rust community";
    let text_x = (DISPLAY_SIZE - (text.len() as u32 * 6)) / 2;
    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    Text::new(text, Point::new(text_x as i32, 190), character_style)
        .draw(display_driver)?;

    Ok(())
}