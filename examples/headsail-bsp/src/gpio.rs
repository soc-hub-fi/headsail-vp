use crate::{
    mmap::{self, GPIO_OUT},
    read_u32, write_u32,
};

pub fn reset_dir() {
    write_u32(mmap::GPIO_DIR, 0);
}

pub fn set_dir_out(gpio_id: usize) {
    write_u32(mmap::GPIO_DIR, 1 << gpio_id);
}

pub fn toggle(gpio_id: usize) {
    // Toggle GPIO
    let mut r = read_u32(GPIO_OUT);
    r ^= 1 << gpio_id;
    write_u32(GPIO_OUT, r);
}
