use std::error::Error;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

// Instruction Set
const WRITE: u8 = 0b0010;
const READ: u8 = 0b0011;
const RDSR: u8 = 0b0101;
const WREN: u8 = 0b0110;

fn main() -> Result<(), Box<dyn Error>>{

    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0)?;

    // Read status
    let result = spi.write(&[RDSR])?;
    println!("{}", result);

    Ok(())
}
