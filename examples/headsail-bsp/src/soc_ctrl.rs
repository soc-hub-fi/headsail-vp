use crate::{mmap, read_u32, write_u32};

pub fn pad0_fn_select(val: u32) {
    let r = read_u32(mmap::PADMUX0);
    write_u32(mmap::PADMUX0, r | val);
}

pub fn ss_enable(ss_bits: u32) {
    write_u32(mmap::SS_RESET_EN, ss_bits);
}

pub fn clk2_set(conf_val: u32) {
    write_u32(mmap::SS_CLK_CTRL2, conf_val);
}

pub fn clk3_set(conf_val: u32) {
    write_u32(mmap::SS_CLK_CTRL3, conf_val);
}
