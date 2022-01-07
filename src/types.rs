// Implement the default "ON" state for each primitive type
pub trait DefaultStates: PartialEq {
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
