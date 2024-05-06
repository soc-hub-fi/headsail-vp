/**
 * Date: 6/5/2024
 * Author: Andreas Stergiopoulos (andreas.stergiopoulos@tuni.fi)
 * 
 * This is the driver for the PULP timer unit. This driver is to 
 * be used in the VIRTUAL PROTOTYPE ONLY.
 * 
 * Documentation: https://github.com/pulp-platform/timer_unit/tree/master
 */

use crate::{mmap::TIMER0_ADDR, write_u32, read_u32};

extern crate bit_field;
use bit_field::BitField;

const TIMER0_CTRL_REG_OFFSET: usize = 0x0;
const TIMER0_COUNTER_REG_OFFSET: usize = 0x8;
const TIMER0_ENABLE_BIT: u32 = 0b0;

#[inline]
pub fn timer0_enable()
{
    // Read register
    let mut reg = read_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET);
    // Make enable bit 1
    reg.set_bit(TIMER0_ENABLE_BIT as usize, true);
    // Write register back
    write_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET, reg as u32);
}

#[inline]
pub fn timer0_disable()
{
    // Read register
    let mut reg = read_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET);
    // Write 0 to bit 0 but leave all other bits untouched
    reg.set_bit(TIMER0_ENABLE_BIT as usize, false);
    // Write register back
    write_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET, reg as u32);
}

#[inline]
#[cfg(debug_assertions)]
pub fn timer0_get_count() -> u32 
{
    return read_u32(TIMER0_ADDR + TIMER0_COUNTER_REG_OFFSET);
}

#[inline]
#[cfg(debug_assertions)]
pub fn timer0_get_ctrl_reg() -> u32
{
    return read_u32(TIMER0_ADDR + TIMER0_CTRL_REG_OFFSET);
}
