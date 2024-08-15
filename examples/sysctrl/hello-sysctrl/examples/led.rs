//! Blinks a LED
//!
//! | Test date     | Status    |
//! | :-            | :-:       |
//! | 2024-08-15    | *Works*   |
#![no_std]
#![no_main]

use headsail_bsp::{read_u32, rt::entry, write_u32};
use panic_halt as _;

// Below addresses are in SysCtrl memory space
mod mmap {
    pub(crate) const GPIO: usize = 0x1a10_1000;
    pub(crate) const GPIO_DIR: usize = GPIO + 0x0;
    pub(crate) const GPIO_OUT: usize = GPIO + 0xc;
    pub(crate) const SOC_CONTROL: usize = 0x1a10_4000;
    pub(crate) const PADMUX0: usize = SOC_CONTROL + 0x10;
}
use mmap::*;

// Number of nops SysCtrl is capable of executing at 30 MHz reference clocks
const NOPS_PER_SEC: usize = match () {
    #[cfg(debug_assertions)]
    // This is an experimentally found value
    () => 2_000_000 / 9,
    #[cfg(not(debug_assertions))]
    // This is just a guess for now (10x debug)
    () => 20_000_000 / 9,
};

#[entry]
fn main() -> ! {
    write_u32(PADMUX0, 0);
    write_u32(GPIO_DIR, 0);

    // Padmux enable GPIO9
    write_u32(PADMUX0, 0x40000);

    // Set GPIO 9 as output
    write_u32(GPIO_DIR, 1 << 9);

    loop {
        unsafe {
            // Toggle GPIO
            let mut r = read_u32(mmap::GPIO_OUT);
            r ^= 1 << 9;
            write_u32(GPIO_OUT, r);

            // 1 second period
            for _ in 0..NOPS_PER_SEC {
                core::arch::asm!("nop");
            }
        }
    }
}
