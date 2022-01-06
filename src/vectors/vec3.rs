use super::{Vector2, Vector4};
use crate::{
    vector::{Swizzable, Vector, VectorDefaults}, types::DefaultState,
};
use core::fmt;
use std::{
    hash::Hash,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Sub, SubAssign},
};

// A simple 3D vector, no simd support what-so-ever
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// Printing
impl<T> fmt::Display for Vector3<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

// Vector trait
impl<T> Vector<T> for Vector3<T>
where
    T: DefaultState,
{
    fn get_unsized(self) -> crate::vector::UnsizedVector<T> {
        crate::vector::UnsizedVector::Vec3(self)
    }
}
impl<T> VectorDefaults for Vector3<T>
where
    T: DefaultState,
{
    const ELEM_COUNT: usize = 3;
}
impl<T> VectorDefaults for &Vector3<T>
where
    T: DefaultState,
{
    const ELEM_COUNT: usize = 3;
}

// Default
impl<T> Default for Vector3<T>
where
    T: DefaultState,
{
    fn default() -> Self {
        Self::ZERO
    }
}

// Implement the vec3 code
#[allow(dead_code)]
impl<T> Vector3<T>
where
    T: DefaultState,
{
    // Defaults
    pub const ZERO: Self = Self { x: T::OFF, y: T::OFF, z: T::OFF };
    pub const X: Self = Self { x: T::ON, y: T::OFF, z: T::OFF };
    pub const Y: Self = Self { x: T::OFF, y: T::ON, z: T::OFF };
    pub const Z: Self = Self { x: T::OFF, y: T::OFF, z: T::ON };
    pub const ONE: Self = Self { x: T::ON, y: T::ON, z: T::ON };
    // Create a new vec4
    pub fn new(f1: T, f2: T, f3: T) -> Self {
        Self { x: f1, y: f2, z: f3 }
    }
}

// Indexer
impl<T> Index<usize> for Vector3<T> {
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => todo!(),
        }
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Vector3<T> {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => todo!(),
        }
    }
}

// Swizzle a vec3
impl<T> Swizzable<T> for Vector3<T>
where
    T: DefaultState,
{
    fn get4(&self, order: [usize; 4]) -> Vector4<T> {
        Vector4::new(self[order[0]], self[order[1]], self[order[2]], self[order[3]])
    }

    fn get3(&self, order: [usize; 3]) -> Vector3<T> {
        Vector3::new(self[order[0]], self[order[1]], self[order[2]])
    }

    fn get2(&self, order: [usize; 2]) -> Vector2<T> {
        Vector2::new(self[order[0]], self[order[1]])
    }
}

// The axii for a vec3
#[derive(Clone, Copy, Debug)]
pub enum Vec3Axis {
    X,
    Y,
    Z,
}

// Get the default axii from the Vec3Axis
impl<T> Vector3<T>
where
    T: DefaultState,
{
    // Get the default value
    pub fn get_default_axis(axis: Vec3Axis) -> Self {
        match axis {
            Vec3Axis::X => Self::X,
            Vec3Axis::Y => Self::Y,
            Vec3Axis::Z => Self::Z,
        }
    }
    // Get the value of the current axis
    pub fn get_axis(&self, axis: Vec3Axis) -> T {
        match axis {
            Vec3Axis::X => self.x,
            Vec3Axis::Y => self.y,
            Vec3Axis::Z => self.z,
        }
    }
}

// Vector3 arithmatics
impl Vector3<f32> {
    // Get the cross product between two vectors
    pub fn cross(self, other: Self) -> Vector3<f32> {
        let a = self;
        let b = other;
        Vector3::new(a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])
    }
}
impl Vector3<f64> {
    // Get the cross product between two vectors
    pub fn cross(self, other: Self) -> Vector3<f64> {
        let a = self;
        let b = other;
        Vector3::new(a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])
    }
}
