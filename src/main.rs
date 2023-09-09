#![no_main]
#![no_std]

// To link the vector table include the hal
use nrf9160_hal as _;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}
