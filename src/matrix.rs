use std::ops::{Index, IndexMut, Mul};

use crate::{
    impl_matrix,
    types::SupportedValue,
    vector::Swizzable,
    vectors::{Vector3, Vector4},
    Quaternion,
};

// A simple f32 matrix made of 4 f32/f64 vectors
// TODO: Turn this into a generic struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4<T>
where
    T: SupportedValue,
{
    pub data: [Vector4<T>; 4],
}

// Default
impl<T> Default for Matrix4x4<T>
where
    T: SupportedValue + Sized,
{
    fn default() -> Self {
        Self::IDENTITY
    }
}

// Indexer
impl<T> Index<usize> for Matrix4x4<T>
where
    T: SupportedValue,
{
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &T {
        let i1 = index / 4;
        let i2 = index % 4;
        let vector = &self.data[i1];
        &vector[i2]
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Matrix4x4<T>
where
    T: SupportedValue,
{
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut T {
        let i1 = index / 4;
        let i2 = index % 4;
        let vector = &mut self.data[i1];
        &mut vector[i2]
    }
}

// Identity matrix available for everyone
impl<T> Matrix4x4<T>
where
    T: SupportedValue,
{
    // Identity matrix
    pub const IDENTITY: Self = Matrix4x4 {
        data: [Vector4::<T>::X, Vector4::<T>::Y, Vector4::<T>::Z, Vector4::<T>::W],
    };
}

impl<T> Matrix4x4<T>
where
    T: SupportedValue,
{
    // Transpose the matrix
    pub fn transpose(&mut self) {
        self.data = self.transposed().data;
    }
    // Return the transpose of this matrix
    pub fn transposed(&self) -> Self {
        let mut output = Self::IDENTITY;
        for x in 0..4 {
            for y in 0..4 {
                let m: &mut T = &mut output[x + y * 4];
                *m = self[y + x * 4];
            }
        }
        output
    }
    // Get the "n" vector
    pub fn get_vec(&self, n: usize) -> &Vector4<T> {
        &self.data[n]
    }
    // Get the "n" vector mutably
    pub fn get_vec_mut(&mut self, n: usize) -> &mut Vector4<T> {
        &mut self.data[n]
    }
}

impl_matrix!(Matrix4x4<f32>, f32);
impl_matrix!(Matrix4x4<f64>, f64);

// Multiply this matrix by another matrix
impl Mul for Matrix4x4<f32> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.mul_mat4x4(rhs)
    }
}