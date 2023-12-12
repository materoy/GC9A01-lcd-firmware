use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::Drawable;
use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::primitives::{Circle, CornerRadii, PrimitiveStyleBuilder, Rectangle, RoundedRectangle, StyledDrawable};
use embedded_graphics::text::Text;

const DISPLAY_SIZE: u32 = 240;
pub fn draw<D: DrawTarget<Color=Rgb565>>(display_driver: &mut D) -> Result<(), D::Error> {
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
