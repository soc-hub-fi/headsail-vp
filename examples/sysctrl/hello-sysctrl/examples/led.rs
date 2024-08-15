//! Blinks a LED
//!
//! | Date              | Status    | Changes   |
//! | :-                | :-:       | :-        |
//! | 2024-08-15        | *Works*   |           |
//! | 2024-08-15T1530   | Untested  | Use HAL   |
#![no_std]
#![no_main]

use headsail_bsp::{
    rt::entry,
    sysctrl::{gpio::Gpio9, soc_ctrl},
};
use hello_sysctrl::NOPS_PER_SEC;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // Set pad9 as GPIO
    soc_ctrl::pad0_fn_select(0);
    soc_ctrl::pad0_fn_select(0x40000);

    let mut gpio9 = Gpio9::new().into_output();

    loop {
        unsafe {
            gpio9.toggle();

            // Wait for 1 second to produce 0.5 Hz period
            for _ in 0..NOPS_PER_SEC {
                core::arch::asm!("nop");
            }
        }
    }
}
