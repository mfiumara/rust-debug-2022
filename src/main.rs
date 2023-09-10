#![no_main]
#![no_std]

// To link the vector table include the hal
use hal::pac;
use nrf52840_hal as hal;

// Logging functionality over RTT
use rtt_target::{rprint, rprintln, rtt_init_print};

use core::panic::PanicInfo;
use cortex_m_rt::entry;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Acquire a reference to the GPIO
    let p = pac::Peripherals::take().unwrap();
    let port1 = hal::gpio::p1::Parts::new(p.P1);

    // The I2C of the nrf52840 on the thingy91, replace if you're using different hardware
    let sda = port1.p1_08.into_floating_input();
    let scl = port1.p1_09.into_floating_input();

    // Instantiate and enable the Two-Wire Interface Peripheral (I2C)
    let mut twim = hal::Twim::new(
        p.TWIM0,
        hal::twim::Pins {
            sda: sda.degrade(),
            scl: scl.degrade(),
        },
        hal::twim::Frequency::K400,
    );
    twim.enable();

    rprintln!("Scanning I2C bus...\r");

    // Print I2C table the header
    rprintln!("     0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f\r");
    rprint!("00: ");
    // Loop over all addresses on the I2C bus
    for i in 1..0xFF {
        if i % 0x10 == 0 {
            rprint!("\r\n{:X}: ", i);
        }

        // We're issuing a simple scan to check if there's an ACK
        // We do not care about the rsult in the buffer but we need to
        // provide a non-empty buffer
        let mut buffer: [u8; 1] = [0xFF];
        match twim.read(i, &mut buffer) {
            Ok(_) => {
                rprint!("{:X} ", i);
            }
            Err(err) => {
                match err {
                    // In case of a NACK we print -- similar to i2cdetect on Linux
                    hal::twim::Error::AddressNack => {
                        rprint!("-- ");
                    }
                    _ => {
                        // Handle other error types if needed
                        rprintln!("Error reading from TWIM: {:?}\r", err);
                        break;
                    }
                }
            }
        }
    }
    rprintln!("\r\nDone!\r");

    loop {}
}
