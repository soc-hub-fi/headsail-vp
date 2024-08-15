use core::marker::PhantomData;

use super::mmap;
use crate::{mask_u32, toggle_u32, unmask_u32};

/// Type-state trait for GPIO in different states
pub trait GpioState {}

pub struct Uninit;
impl GpioState for Uninit {}

pub struct Input;
impl GpioState for Input {}

pub struct Output;
impl GpioState for Output {}

pub struct Gpio<const IDX: u32, State: GpioState = Uninit> {
    _pd: PhantomData<State>,
}

pub type Gpio9 = Gpio<9, Uninit>;

#[repr(u32)]
enum Dir {
    In = 0,
    Out = 1,
}

impl<const IDX: u32> Gpio<IDX, Uninit> {
    // TODO: should require proof that the respective pad has been initialized
    // TODO: perhaps pad0_fn_select should return the new GPIO
    pub fn new() -> Self {
        Self { _pd: PhantomData }
    }

    pub fn into_input(self) -> Gpio<IDX, Input> {
        mask_u32(mmap::GPIO_DIR, Dir::In as u32);

        Gpio { _pd: PhantomData }
    }

    pub fn into_output(self) -> Gpio<IDX, Output> {
        mask_u32(mmap::GPIO_DIR, Dir::Out as u32);

        Gpio { _pd: PhantomData }
    }
}

impl<const IDX: u32> Gpio<IDX, Output> {
    pub fn toggle(&mut self) {
        toggle_u32(mmap::GPIO_OUT, 1 << IDX);
    }

    pub fn set_high(&mut self) {
        mask_u32(mmap::GPIO_OUT, 1 << IDX);
    }

    pub fn set_low(&mut self) {
        unmask_u32(mmap::GPIO_OUT, 1 << IDX);
    }
}
