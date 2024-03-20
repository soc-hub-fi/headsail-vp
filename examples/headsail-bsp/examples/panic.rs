#![no_std]
#![no_main]

use headsail_bsp::{rt::entry, uart::uart_write};

#[entry]
fn main() -> ! {
    panic!("panic");
    loop {}
}
