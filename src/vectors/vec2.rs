use super::{Vector3, Vector4};
use crate::{
    vector::{Swizzable, Vector, VectorDefaults}, consts,
};
use std::{
    fmt,
    hash::Hash,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Sub, SubAssign},
};

// A simple 2D vector, no simd support what-so-ever
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

// Printing
impl<T> fmt::Display for Vector2<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

// Vector trait
impl<T> Vector<T> for Vector2<T>
where
    T: num_traits::Num,
{
    fn get_unsized(self) -> crate::vector::UnsizedVector<T> {
        crate::vector::UnsizedVector::<T>::Vec2(self)
    }
}
impl<T> VectorDefaults for Vector2<T>
where
    T: num_traits::Num,
{
    const ELEM_COUNT: usize = 2;
    const ZERO: Self = consts::vec2(T::zero(), T::zero());
    const ONE: Self = consts::vec2(T::one(), T::one());
}

// Default
impl<T> Default for Vector2<T>
where
    T: num_traits::Num,
{
    fn default() -> Self {
        Self::ZERO
    }
}

// Implement the vec3 code
#[allow(dead_code)]
impl<T> Vector2<T>
where
    T: num_traits::Num,
{
    // Defaults
    pub const ZERO: Self = Self { x: T::zero(), y: T::OFF };
    pub const X: Self = Self { x: T::ON, y: T::OFF };
    pub const Y: Self = Self { x: T::OFF, y: T::ON };
    pub const ONE: Self = Self { x: T::ON, y: T::ON };
    // Create a new vec4
    pub fn new(f1: T, f2: T) -> Self {
        Self { x: f1, y: f2 }
    }
}

// Indexer
impl<T> Index<usize> for Vector2<T> {
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => todo!(),
        }
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Vector2<T> {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => todo!(),
        }
    }
}

// Swizzle a vec2
impl<T> Swizzable<T> for Vector2<T>
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

// The axii for a vec2
#[derive(Clone, Copy, Debug)]
pub enum Vec2Axis {
    X,
    Y,
}

// Get the default axii from the Vec2Axis
impl<T> Vector2<T>
where
    T: num_traits::Num,
{
    // Get the default value
    pub fn get_default_axis(axis: Vec2Axis) -> Self {
        match axis {
            Vec2Axis::X => Self::X,
            Vec2Axis::Y => Self::Y,
        }
    }
    // Get the value of the current axis
    pub fn get_axis(&self, axis: Vec2Axis) -> T {
        match axis {
            Vec2Axis::X => self.x,
            Vec2Axis::Y => self.y,
        }
    }
}
