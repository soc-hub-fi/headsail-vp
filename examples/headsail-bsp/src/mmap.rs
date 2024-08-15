#[cfg(feature = "hpc")]
pub(crate) const UART0_ADDR: usize = 0x1FFF00000;
#[cfg(not(feature = "hpc"))]
pub(crate) const UART0_ADDR: usize = 0xFFF00000;
pub(crate) const UART0_THR: usize = UART0_ADDR + 0;

// NOTE: (20240614 vaino-waltteri.granat@tuni.fi) This applies to renode NS16550 uart, but might not apply to headsail ASIC
pub(crate) const UART_DATA_READY_OFFSET: usize = 5;

#[cfg(feature = "asic")]
mod asic_uart {
    use super::UART0_ADDR;

    pub(crate) const UART0_DIV_LSB: usize = UART0_ADDR + 0;
    pub(crate) const UART0_DIV_MSB: usize = UART0_ADDR + 1;
    pub(crate) const UART0_INTERRUPT_ENABLE: usize = UART0_ADDR + 1;
    pub(crate) const UART0_FIFO_CONTROL: usize = UART0_ADDR + 2;
    pub(crate) const UART0_LINE_CONTROL: usize = UART0_ADDR + 3;
    pub(crate) const UART0_MODEM_CONTROL: usize = UART0_ADDR + 4;
    pub(crate) const UART0_LINE_STATUS: usize = UART0_ADDR + 5;
}
#[cfg(feature = "asic")]
pub(crate) use self::asic_uart::*;

pub(crate) const TIMER0_ADDR: usize = 0x5_0000;
pub(crate) const TIMER1_ADDR: usize = 0x5_0010;
pub(crate) const TIMER2_ADDR: usize = 0x5_0020;
pub(crate) const TIMER3_ADDR: usize = 0x5_0030;

/// SysCtrl-specific memory maps
#[cfg(feature = "sysctrl")]
mod sysctrl {
    pub(crate) const SYSCTRL_ADDR: usize = 0x1a10_0000;
    pub(crate) const GPIO_ADDR: usize = SYSCTRL_ADDR + 0x1000;
    pub(crate) const GPIO_DIR: usize = GPIO_ADDR + 0x0;
    pub(crate) const GPIO_OUT: usize = GPIO_ADDR + 0xc;

    pub(crate) const SOC_CONTROL_ADDR: usize = SYSCTRL_ADDR + 0x4000;
    pub(crate) const PADMUX0: usize = SOC_CONTROL_ADDR + 0x10;
    pub(crate) const SS_RESET_EN: usize = SOC_CONTROL_ADDR + 0xb0;
    pub(crate) const SS_CLK_CTRL2: usize = SOC_CONTROL_ADDR + 0x9c;
    pub(crate) const SS_CLK_CTRL3: usize = SOC_CONTROL_ADDR + 0xb8;

    pub(crate) const RAM0: usize = 0x9008_0000;
    pub(crate) const RAM1: usize = 0x9028_0000;
    pub(crate) const RAM2: usize = 0x9048_0000;
    pub(crate) const RAM3: usize = 0x9068_0000;
}
#[cfg(feature = "sysctrl")]
pub(crate) use self::sysctrl::*;

/// Length for ram_0, ram_1, ram_2, ram_3
pub(crate) const RAM_LEN: usize = 0x8000;
