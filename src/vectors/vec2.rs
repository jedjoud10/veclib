use super::{Vector3, Vector4};
use crate::{
    types::SupportedValue,
    vector::{Swizzable, Vector, VectorElemCount},
};
use std::{
    fmt,
    hash::Hash,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Sub, SubAssign},
};

// A simple 2D vector, no simd support what-so-ever
#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(C)]
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
impl<T> Vector<T> for Vector2<T> {
    fn get_unsized(self) -> crate::vector::UnsizedVector<T>
    where
        T: PartialEq + SupportedValue,
    {
        crate::vector::UnsizedVector::<T>::Vec2(self)
    }

    fn as_ptr(&self) -> *const T {
        &self.x
    }

    fn as_ptr_mut(&mut self) -> *mut T {
        &mut self.x
    }
}
impl<T> VectorElemCount for Vector2<T> {
    const ELEM_COUNT: usize = 2;
}

impl<T> VectorElemCount for &Vector2<T> {
    const ELEM_COUNT: usize = 2;
}

// Default
impl<T: Default> Default for Vector2<T> {
    fn default() -> Self {
        Self { x: T::default(), y: T::default() }
    }
}

pub const fn vec2<T>(f1: T, f2: T) -> Vector2<T> {
    Vector2::new(f1, f2)
}

impl<T> Vector2<T> {
    // Create a new vec2
    pub const fn new(f1: T, f2: T) -> Self {
        Self { x: f1, y: f2 }
    }
}

// Implement the vec3 code
#[allow(dead_code)]
impl<T: SupportedValue> Vector2<T> {
    // Defaults
    pub const ZERO: Self = Self { x: T::ZERO, y: T::ZERO };
    pub const X: Self = Self { x: T::ONE, y: T::ZERO };
    pub const Y: Self = Self { x: T::ZERO, y: T::ONE };
    pub const ONE: Self = Self { x: T::ONE, y: T::ONE };
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
impl<T: Clone + Copy> Swizzable<T> for Vector2<T> {
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
impl<T: SupportedValue> Vector2<T> {
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

crate::setup_any_vector_operations!(Vector2<u8>, T, u8);
crate::setup_any_vector_operations!(Vector2<u16>, T, u16);
crate::setup_any_vector_operations!(Vector2<u32>, T, u32);
crate::setup_any_vector_operations!(Vector2<u64>, T, u64);
crate::setup_any_vector_operations!(Vector2<u128>, T, u128);
crate::setup_any_vector_operations!(Vector2<i8>, T, i8);
crate::setup_any_vector_operations!(Vector2<i16>, T, i16);
crate::setup_any_vector_operations!(Vector2<i32>, T, i32);
crate::setup_any_vector_operations!(Vector2<i64>, T, i64);
crate::setup_any_vector_operations!(Vector2<i128>, T, i128);
crate::setup_any_vector_operations!(Vector2<f32>, T, f32);
crate::setup_any_vector_operations!(Vector2<f64>, T, f64);

crate::setup_floating_vector_operations!(Vector2<f32>, T, f32);
crate::setup_floating_vector_operations!(Vector2<f64>, T, f64);
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
