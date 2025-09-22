#![no_std]
#![no_main]

use cortex_m as _;
use defmt::println;
use defmt_rtt as _;
use fuel_monitor::{FuelLevel, FuelMonitor};

const GPIO_P0_BASE: *mut u32 = 0x5000_0500 as *mut u32;

const OUTSET_OFFSET: isize = 0x08;
const OUTCLR_OFFSET: isize = 0x0C;
const DIRSET_OFFSET: isize = 0x18;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut monitor = FuelMonitor::new();
    // monitor.insert(FuelLevel::with_litres(1.0));
    let minimum = monitor.min();
    let maximum = monitor.max();

    unsafe {
        GPIO_P0_BASE
            .byte_offset(DIRSET_OFFSET)
            .write_volatile(1 << 13);
    }

    println!("Hello, world! min={}, max={}", minimum, maximum);

    loop {
        unsafe {
            GPIO_P0_BASE
                .byte_offset(OUTSET_OFFSET)
                .write_volatile(1 << 13);
        }

        cortex_m::asm::delay(1_000_000);

        unsafe {
            GPIO_P0_BASE
                .byte_offset(OUTCLR_OFFSET)
                .write_volatile(1 << 13);
        }

        cortex_m::asm::delay(1_000_000);
    }
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
