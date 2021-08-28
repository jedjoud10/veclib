// Implement the default "ON" state for each primitive type
pub trait DefaultStates {
    fn off() -> Self;
    fn on() -> Self;
}

// A floating point precision trait
pub trait FloatingPoint {}

// I32
impl DefaultStates for i32 {
    fn off() -> Self {
        0
    }
    fn on() -> Self {
        1
    }
}

// I64
impl DefaultStates for i64 {
    fn off() -> Self {
        0
    }
    fn on() -> Self {
        1
    }
}

// F32
impl DefaultStates for f32 {
    fn off() -> Self {
        0.0
    }
    fn on() -> Self {
        1.0
    }
}

// F64
impl DefaultStates for f64 {
    fn off() -> Self {
        0.0
    }
    fn on() -> Self {
        1.0
    }
}

// BOOL
impl DefaultStates for bool {
    fn off() -> Self {
        false
    }
    fn on() -> Self {
        true
    }
}
