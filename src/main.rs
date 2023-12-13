use crate::draw::draw;
use crate::rpi::rpi;

mod rpi;
mod draw;

fn main() -> ! {
    let mut driver = rpi();
    draw(&mut driver).unwrap();

    loop {}
}
