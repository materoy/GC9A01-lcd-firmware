use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::pixelcolor::{Rgb565, RgbColor};
use embedded_graphics::primitives::{Circle, PrimitiveStyleBuilder, StyledDrawable};

const DISPLAY_SIZE: u32 = 240;

pub fn draw<D: DrawTarget<Color=Rgb565>>(display_driver: &mut D) -> Result<(), D::Error> {
    let style = PrimitiveStyleBuilder::new()
        .stroke_width(8)
        .stroke_color(Rgb565::GREEN)
        .build();

    let padding = 20;
    // Outline Circle
    Circle::new(Point::new(padding, padding), DISPLAY_SIZE - padding as u32 * 2)
        .draw_styled(&style, display_driver)?;

    Ok(())
}
