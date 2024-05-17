/*!
 * Date: 6/5/2024
 * Author: Andreas Stergiopoulos (andreas.stergiopoulos@tuni.fi)
 *
 * This is the driver for the PULP APB Timer. The documentation
 * for this peripheral is the register map provided in the Headsail
 * gitlab pages.
 */
use crate::{mmap::*, read_u32, write_u32};

use bit_field::BitField;

pub struct APBTimer<const BASE_ADDRESS: usize>;

impl<const BASE_ADDRESS: usize> APBTimer<BASE_ADDRESS>{
    const TIMER_COUNTER_REG_OFFSET: usize = 0x0;
    const TIMER_CTRL_REG_OFFSET: usize = 0x4;
    const TIMER_CMP_REG_OFFSET: usize = 0x8;
    const TIMER_ENABLE_BIT: usize = 0b0;

    /**
     * Enables the timer (starts counting).
     */
    #[inline]
    pub fn enable() {
        // Read register
        let mut reg = read_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET);
        // Make enable bit 1
        reg.set_bit(Self::TIMER_ENABLE_BIT, true);
        // Write register back
        write_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET, reg);
    }

    /**
     * Disables the timer (stops counting).
     */
    #[inline]
    pub fn disable() {
        // Read register
        let mut reg = read_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET);
        // Write 0 to bit 0 but leave all other bits untouched
        reg.set_bit(Self::TIMER_ENABLE_BIT as usize, false);
        // Write register back
        write_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET, reg);
    }

    /**
     * Returns the timer counter (tick value).
     */
    #[inline]
    pub fn get_count() -> u32 {
        return read_u32(BASE_ADDRESS + Self::TIMER_COUNTER_REG_OFFSET);
    }

    #[inline]
    #[cfg(debug_assertions)]
    pub fn get_ctrl_reg() -> u32 {
        return read_u32(BASE_ADDRESS + Self::TIMER_CTRL_REG_OFFSET);
    }
}

pub type Timer0 = APBTimer::<TIMER0_ADDR>;
pub type Timer1 = APBTimer::<TIMER1_ADDR>;
pub type Timer2 = APBTimer::<TIMER2_ADDR>;
pub type Timer3 = APBTimer::<TIMER3_ADDR>;
