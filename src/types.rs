use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// Implement the default "ON" state for each primitive type
pub trait DefaultStates:
    Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = Self> + Add<Output = Self> + Mul<Output = Self> + Div<Output = Self> + SubAssign + AddAssign + MulAssign + DivAssign
{
    const OFF: Self;
    const ON: Self;
}

// Integer types
crate::impl_default_state!(u8);
crate::impl_default_state!(u16);
crate::impl_default_state!(u32);
crate::impl_default_state!(u64);
crate::impl_default_state!(u128);
crate::impl_default_state!(i8);
crate::impl_default_state!(i16);
crate::impl_default_state!(i32);
crate::impl_default_state!(i64);
crate::impl_default_state!(i128);

// A square root trait because I need it
pub trait SqrtRootable {
    fn sqrt(self) -> Self;
}
impl SqrtRootable for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl SqrtRootable for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

// F32
impl DefaultStates for f32 {
    const OFF: Self = 0.0;
    const ON: Self = 1.0;
}

// F64
impl DefaultStates for f64 {
    const OFF: Self = 0.0;
    const ON: Self = 1.0;
}

// BOOL
impl DefaultStates for bool {
    const OFF: Self = false;
    const ON: Self = true;
}
