use crate::{types::DefaultStates, Vector4};

// A quaternion that represents a rotation
pub struct Quaternion<T> {
    data: Vector4<T>,
}

// Default
impl<T> Default for Quaternion<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    fn default() -> Self {
        Self::IDENTITY
    }
}

// Da code
impl<T> Quaternion<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    // Constants
    pub const IDENTITY: Quaternion<T> = Quaternion { data: Vector4::<T>::W };
}
