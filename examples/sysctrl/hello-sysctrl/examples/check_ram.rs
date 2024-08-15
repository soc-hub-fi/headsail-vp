//! Turns on the interconnect, and validates that SRAM memory banks are usable
//!
//! On VP these are the SRAM, length = 0x8000 (32 KiB):
//!
//! | Name  | Address (SysCtrl) |
//! | :-:   | :-                |
//! | ram_0 | 0x9008_0000       |
//! | ram_1 | 0x9028_0000       |
//! | ram_2 | 0x9048_0000       |
//! | ram_3 | 0x9068_0000       |
//!
//! HPC adds the ext. bit, i.e., 0x1_9008_0000 for ram_0.
//!
//! | Test date     | Status    |
//! | :-            | :-:       |

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use headsail_bsp::{
    read_u8,
    rt::entry,
    sysctrl::{
        gpio::{self, Gpio, Gpio9},
        soc_ctrl,
    },
    write_u8,
};
use hello_sysctrl::NOPS_PER_SEC;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let mut gpio9: Gpio<9, gpio::Output> = unsafe { Gpio9::<gpio::Output>::steal() };

    loop {
        // High for 0.2 seconds
        gpio9.set_high();
        for _ in 0..(NOPS_PER_SEC / 5) {
            unsafe { core::arch::asm!("nop") };
        }

        // Low for 0.8 seconds
        gpio9.set_high();
        for _ in 0..(4 * NOPS_PER_SEC / 5) {
            unsafe { core::arch::asm!("nop") };
        }
    }
}

/// Length for ram_0, ram_1, ram_2, ram_3
const RAM_LEN: usize = 0x8000;
pub(crate) const RAM0: usize = 0x9008_0000;
pub(crate) const RAM1: usize = 0x9028_0000;
pub(crate) const RAM2: usize = 0x9048_0000;
pub(crate) const RAM3: usize = 0x9068_0000;

fn check_ram(start: usize) {
    for addr in start..(start + RAM_LEN) {
        let write = (addr % 256) as u8;
        unsafe { write_u8(addr, write) };
    }

    for addr in start..(start + RAM_LEN) {
        let wrote = (addr % 256) as u8;
        let read = unsafe { read_u8(addr) };
        assert_eq!(wrote, read);
    }
}

#[entry]
fn main() -> ! {
    // Enable interconnect
    let icn_bit = 1 << 5;
    soc_ctrl::ss_enable(icn_bit);

    // Configure ICN clocks
    let conf_val = 0b1001 << 8;
    soc_ctrl::clk2_set(conf_val);

    // Set pad9 as GPIO
    let pads = unsafe { soc_ctrl::Pads::steal() };
    let mut gpio9 = pads.p9.into_gpio().into_output();

    check_ram(RAM0);
    check_ram(RAM1);
    check_ram(RAM2);
    check_ram(RAM3);

    loop {
        gpio9.toggle();

        // Wait for 1 second to produce 0.5 Hz period
        for _ in 0..NOPS_PER_SEC {
            unsafe { core::arch::asm!("nop") };
        }
    }
}
