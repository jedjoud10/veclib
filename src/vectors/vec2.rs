use super::{Vector3, Vector4};
use crate::{types::DefaultStates, vector::{Swizzable, Vector}};
use std::{fmt, hash::Hash, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign, BitAnd, BitOr, BitXor,  Not, BitAndAssign, BitOrAssign, BitXorAssign}};

// A simple 2D vector, no simd support what-so-ever
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T, 
    pub y: T,
}

// Printing
impl<T> fmt::Display for Vector2<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

// Vector trait
impl<T> Vector<T> for Vector2<T> {
    const ELEM_COUNT: usize = 2;
}

impl<T> Vector<T> for &Vector2<T> {
    const ELEM_COUNT: usize = 2;
}

// Default
impl<T> Default for Vector2<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    fn default() -> Self {
        Self::ZERO
    }
}

// Implement the vec3 code
#[allow(dead_code)]
impl<T> Vector2<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    // Defaults
    pub const ZERO: Self = Self {
        x: T::OFF,
        y: T::OFF,
    };
    pub const X: Self = Self {
        x: T::ON,
        y: T::OFF,
    };
    pub const Y: Self = Self {
        x: T::OFF,
        y: T::ON,
    };
    pub const ONE: Self = Self {
        x: T::ON,
        y: T::ON,
    };
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
            _ => todo!()
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
            _ => todo!()
        }
    }
}

// Swizzle a vec2
impl<T> Swizzable<T> for Vector2<T>
where
    T: DefaultStates + Clone + Copy + Sized,
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
pub enum Vec2Axis {
    X,
    Y,
}

// Get the default axii from the Vec2Axis
impl<T> Vector2<T> where T: DefaultStates + Clone + Copy + Sized {
    // Get the default value
    pub fn get_default_axis(axis: &Vec2Axis) -> Self {
        match axis {
            Vec2Axis::X => Self::X,
            Vec2Axis::Y => Self::Y,
        }
    }
}

// Eq and Hash for int types
crate::impl_eq_hash!(Vector2<i16>);
crate::impl_eq_hash!(Vector2<i32>);
crate::impl_eq_hash!(Vector2<i64>);
crate::impl_eq_hash!(Vector2<i128>);
crate::impl_eq_hash!(Vector2<u16>);
crate::impl_eq_hash!(Vector2<u32>);
crate::impl_eq_hash!(Vector2<u64>);
crate::impl_eq_hash!(Vector2<u128>);

// Run the macros
crate::setup_add!(Vector2<T>, T);
crate::setup_sub!(Vector2<T>, T);
crate::setup_mul!(Vector2<T>, T);
crate::setup_div!(Vector2<T>, T);
crate::setup_neg!(Vector2<T>, T);
crate::setup_vector_arithmatics!(Vector2<f32>, T, f32);
crate::setup_vector_arithmatics!(Vector2<f64>, T, f64);
crate::impl_elem_wise_comparison!(Vector2<T>, T, Vector2<bool>);

// Dear lord
// I deeply apologize for this
// Floating point to floating point
crate::impl_from_vec2!(Vector2<f64>, f64, f32);
crate::impl_from_vec2!(Vector2<f32>, f32, f64);
// Integers to floating point
crate::impl_from_vec2!(Vector2<f32>, f32, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
crate::impl_from_vec2!(Vector2<f64>, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
// Integers to integers
crate::impl_from_vec2!(Vector2<i8>, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec2!(Vector2<i16>, i16, i8, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec2!(Vector2<i32>, i32, i8, i16, i64, i128, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec2!(Vector2<i64>, i64, i8, i16, i32, i128, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec2!(Vector2<i128>, i128, i8, i16, i32, i64, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec2!(Vector2<u8>, u8, i8, i16, i32, i64, i128, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec2!(Vector2<u16>, u16, i8, i16, i32, i64, i128, u8, u32, u64, u128, f32, f64);
crate::impl_from_vec2!(Vector2<u32>, u32, i8, i16, i32, i64, i128, u8, u16, u64, u128, f32, f64);
crate::impl_from_vec2!(Vector2<u64>, u64, i8, i16, i32, i64, i128, u8, u16, u32, u128, f32, f64);
crate::impl_from_vec2!(Vector2<u128>, u128, i8, i16, i32, i64, i128, u8, u16, u32, u64, f32, f64);
