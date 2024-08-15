//! Blinks a LED
//!
//! | Test date     | Status    |
//! | :-            | :-:       |
//! | 2024-08-15    | *Works*   |
#![no_std]
#![no_main]

use headsail_bsp::{
    rt::entry,
    sysctrl::{gpio, soc_ctrl},
};
use hello_sysctrl::NOPS_PER_SEC;
use panic_halt as _;

#[entry]
fn main() -> ! {
    soc_ctrl::pad0_fn_select(0);
    gpio::reset_dir();

    // Padmux enable GPIO9
    soc_ctrl::pad0_fn_select(0x40000);

    // Set GPIO 9 as output
    gpio::set_dir_out(9);

    loop {
        unsafe {
            gpio::toggle(9);

            // Wait for 1 second to produce 0.5 Hz period
            for _ in 0..NOPS_PER_SEC {
                core::arch::asm!("nop");
            }
        }
    }
}
