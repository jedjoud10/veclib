use std::ops::{Index, IndexMut};

use crate::{Matrix4x4, Vector4, types::DefaultStates};

// A quaternion that represents a rotation
#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T> {
    data: Vector4<T>,
}

// Default
impl<T> Default for Quaternion<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    fn default() -> Self {
        Self::default_identity()
    }
}

// Indexer
impl<T> Index<usize> for Quaternion<T>
where
    T: DefaultStates + Clone + Copy,
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
    T: DefaultStates + Clone + Copy,
{
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Da code
impl<T> Quaternion<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    // Defaults
    pub fn default_identity() -> Self {
        Quaternion {
            data: Vector4::<T>::default_w(),
        }
    }
}