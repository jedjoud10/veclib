use crate::Vector4;

// A quaternion that represents a rotation
pub struct Quaternion<T> {
    data: Vector4<T>,
}