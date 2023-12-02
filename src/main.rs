use std::error::Error;
use rppal::spi::{Bus, Mode, Segment, SlaveSelect, Spi};

// Instruction Set
const WRITE: u8 = 0b0010;
const READ: u8 = 0b0011;
const RDSR: u8 = 0b0101;
const WREN: u8 = 0b0110;

const WIP: u8 = 1;

fn main() -> Result<(), Box<dyn Error>>{

    let mut spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0)?;

    // Set the write enable latch using the WREN instruction. This is required
    // before any data can be written. The write enable latch is automatically
    // reset after a WRITE instruction is successfully executed.
    spi.write(&[WREN])?;

    // Use the WRITE instruction to select memory address 0 and write 5 bytes
    // (1, 2, 3, 4, 5). Addresses are specified as 24-bit values, but the 7 most
    // significant bits are ignored.
    spi.write(&[WRITE, 0, 0, 0, 1, 2, 3, 4, 5])?;

    // Read the STATUS register by writing the RDSR instruction, and then reading
    // a single byte. Loop until the WIP bit is set to 0, indicating the write
    // operation is completed. transfer_segments() will keep the Slave Select line
    // active until both segments have been transferred.
    let mut buffer = [0u8; 1];
    loop {
        spi.transfer_segments(&[
            Segment::with_write(&[RDSR]),
            Segment::with_read(&mut buffer),
        ])?;

        if buffer[0] & WIP == 0 {
            break;
        }
    }

    // Use the READ instruction to select memory address 0, specified as a 24-bit
    // value, and then read 5 bytes.
    let mut buffer = [0u8; 5];
    spi.transfer_segments(&[
        Segment::with_write(&[READ, 0, 0, 0]),
        Segment::with_read(&mut buffer),
    ])?;

    println!("Bytes read: {:?}", buffer);

    Ok(())
}
