#[allow(dead_code)]

#[cfg(feature = "rpi")]
use crate::rpi::rpi;
#[cfg(feature = "pc")]
use crate::simulator::simulator;

mod simulator;
mod rpi;
mod draw;

fn main() {
    #[cfg(feature = "rpi")]
    rpi();
    #[cfg(feature = "pc")]
    simulator();
}
