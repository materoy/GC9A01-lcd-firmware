use embedded_graphics::geometry::Size;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use crate::draw::draw;

pub fn simulator() {
    let mut simulator_display = SimulatorDisplay::<Rgb565>::new(Size::new(240, 240));
    let output_settings = OutputSettingsBuilder::new()
        .build();
    draw(&mut simulator_display).unwrap();
    Window::new("Sim", &output_settings).show_static(&simulator_display);
}