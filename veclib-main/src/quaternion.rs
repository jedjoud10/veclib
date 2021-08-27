use crate::{Vector4, types::DefaultStates};

// A quaternion that represents a rotation
pub struct Quaternion<T> {
    data: Vector4<T>,
}

// Da code
impl<T> Quaternion<T> where T: DefaultStates + Clone + Copy + Sized {
    // Constants
    pub const IDENTITY: Quaternion<T> = Quaternion { data: Vector4::<T>::W };
}