use std::ops::{Index, IndexMut, Mul};

use crate::{types::SupportedValue, Swizzable, Vector3, Vector4, impl_quaternion};

// A quaternion that represents a rotation
#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T> {
    data: Vector4<T>,
}

// Default
impl<T> Default for Quaternion<T>
where
    T: SupportedValue + Sized,
{
    fn default() -> Self {
        Self::IDENTITY
    }
}

// Indexer
impl<T> Index<usize> for Quaternion<T>
where
    T: SupportedValue,
{
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Quaternion<T>
where
    T: SupportedValue,
{
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// The order of the angles operations when creating a Quaternion from euler angles
pub enum EulerAnglesOrder {
    XYZ,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
}

impl<T> Quaternion<T>
    where T: SupportedValue
{    
    // Identity
    pub const IDENTITY: Self = Self { data: Vector4::<T>::W };
}

impl_quaternion!(Quaternion<f32>, f32);
impl_quaternion!(Quaternion<f64>, f64);