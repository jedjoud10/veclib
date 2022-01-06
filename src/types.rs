use std::process::Output;

use crate::{impl_default_state, impl_float};

// A default state trait that can have a ON or OFF state
pub trait DefaultState {
    const ON: Self;
    const OFF: Self;
}

impl_default_state!(u8);
impl_default_state!(u16);
impl_default_state!(u32);
impl_default_state!(u64);
impl_default_state!(u128);
impl_default_state!(i8);
impl_default_state!(i16);
impl_default_state!(i32);
impl_default_state!(i64);
impl_default_state!(i128);

impl_default_state!(usize);
impl_default_state!(isize);

// A float type
pub trait Float
    where Self: Sized + std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> 
{
    // Get the sqrt of the float value
    fn _sqrt(self) -> Self;
}

impl_float!(f32);
impl_float!(f64);