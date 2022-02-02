// Implement the default "ONE" and "ZERO" values
pub trait SupportedValue: PartialEq + Clone + Copy {
    const ZERO: Self;
    const ONE: Self;
}

// Integer types
crate::impl_default_state!(usize);
crate::impl_default_state!(isize);
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

// F32
impl SupportedValue for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

// F64
impl SupportedValue for f64 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
}

// BOOL
impl SupportedValue for bool {
    const ZERO: Self = false;
    const ONE: Self = true;
}
