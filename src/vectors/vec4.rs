use super::{Vector2, Vector3};
use crate::{
    vector::{Swizzable, Vector, VectorDefaults},
};
use std::{
    fmt::{self},
    hash::Hash,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Sub, SubAssign},
};

// A simple 4D vector, no simd support what-so-ever
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

// Printing
impl<T> fmt::Display for Vector4<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

// Vector trait
impl<T> Vector<T> for Vector4<T>
where
    T: num_traits::Num,
{
    fn get_unsized(self) -> crate::vector::UnsizedVector<T> {
        crate::vector::UnsizedVector::Vec4(self)
    }
}
impl<T> VectorDefaults for Vector4<T>
where
    T: num_traits::Num,
{
    const ELEM_COUNT: usize = 4;
}
impl<T> VectorDefaults for &Vector4<T>
where
    T: num_traits::Num,
{
    const ELEM_COUNT: usize = 4;
}

// Default
impl<T> Default for Vector4<T>
where
    T: num_traits::Num,
{
    fn default() -> Self {
        Self::ZERO
    }
}

// Implement the vec4 code
#[allow(dead_code)]
impl<T> Vector4<T>
where
    T: num_traits::Num,
{
    // Defaults
    pub const ZERO: Self = Self {
        x: T::OFF,
        y: T::OFF,
        z: T::OFF,
        w: T::OFF,
    };
    pub const X: Self = Self {
        x: T::ON,
        y: T::OFF,
        z: T::OFF,
        w: T::OFF,
    };
    pub const Y: Self = Self {
        x: T::OFF,
        y: T::ON,
        z: T::OFF,
        w: T::OFF,
    };
    pub const Z: Self = Self {
        x: T::OFF,
        y: T::OFF,
        z: T::ON,
        w: T::OFF,
    };
    pub const W: Self = Self {
        x: T::OFF,
        y: T::OFF,
        z: T::OFF,
        w: T::ON,
    };
    pub const ONE: Self = Self {
        x: T::ON,
        y: T::ON,
        z: T::ON,
        w: T::ON,
    };
    // Create a new vec4
    pub fn new(f1: T, f2: T, f3: T, f4: T) -> Self {
        Self { x: f1, y: f2, z: f3, w: f4 }
    }
}

// Indexer
impl<T> Index<usize> for Vector4<T> {
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => todo!(),
        }
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Vector4<T> {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => todo!(),
        }
    }
}

// Swizzle a vec4
impl<T> Swizzable<T> for Vector4<T>
where
    T: num_traits::Num,
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

// The axii for a vec4
#[derive(Clone, Copy, Debug)]
pub enum Vec4Axis {
    X,
    Y,
    Z,
    W,
}

// Get the default axii from the Vec4Axis
impl<T> Vector4<T>
where
    T: num_traits::Num,
{
    // Get the default value
    pub fn get_default_axis(axis: Vec4Axis) -> Self {
        match axis {
            Vec4Axis::X => Self::X,
            Vec4Axis::Y => Self::Y,
            Vec4Axis::Z => Self::Z,
            Vec4Axis::W => Self::W,
        }
    }
    // Get the value of the current axis
    pub fn get_axis(&self, axis: Vec4Axis) -> T {
        match axis {
            Vec4Axis::X => self.x,
            Vec4Axis::Y => self.y,
            Vec4Axis::Z => self.z,
            Vec4Axis::W => self.w,
        }
    }
}
