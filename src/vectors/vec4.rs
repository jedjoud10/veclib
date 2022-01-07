use super::{Vector2, Vector3};
use crate::{
    types::SupportedValue,
    vector::{Swizzable, Vector, VectorElemCount},
};
use std::{
    fmt::{self},
    hash::Hash,
    ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Sub, SubAssign},
};

// A simple 4D vector, no simd support what-so-ever
#[derive(PartialEq, Debug, Clone, Copy)]
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
impl<T> Vector<T> for Vector4<T> {
    fn get_unsized(self) -> crate::vector::UnsizedVector<T> where T: PartialEq + SupportedValue {
        crate::vector::UnsizedVector::Vec4(self)
    }
}
impl<T> VectorElemCount for Vector4<T> {
    const ELEM_COUNT: usize = 4;
}

// Default
impl<T: Default> Default for Vector4<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
            w: T::default(),
        }
    }
}

impl<T> Vector4<T> {
    // Create a new vec4
    pub const fn new(f1: T, f2: T, f3: T, f4: T) -> Self {
        Self { x: f1, y: f2, z: f3, w: f4 }
    }
}

// Implement the vec4 code
#[allow(dead_code)]
impl<T: SupportedValue> Vector4<T> {
    // Defaults
    pub const ZERO: Self = Self {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ZERO,
    };
    pub const X: Self = Self {
        x: T::ONE,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ZERO,
    };
    pub const Y: Self = Self {
        x: T::ZERO,
        y: T::ONE,
        z: T::ZERO,
        w: T::ZERO,
    };
    pub const Z: Self = Self {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ONE,
        w: T::ZERO,
    };
    pub const W: Self = Self {
        x: T::ZERO,
        y: T::ZERO,
        z: T::ZERO,
        w: T::ONE,
    };
    pub const ONE: Self = Self {
        x: T::ONE,
        y: T::ONE,
        z: T::ONE,
        w: T::ONE,
    };    
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
impl<T: Clone + Copy> Swizzable<T> for Vector4<T> {
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
impl<T: SupportedValue> Vector4<T> {
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

// Eq and Hash for int types
crate::impl_eq_hash!(Vector4<i16>);
crate::impl_eq_hash!(Vector4<i32>);
crate::impl_eq_hash!(Vector4<i64>);
crate::impl_eq_hash!(Vector4<i128>);
crate::impl_eq_hash!(Vector4<u16>);
crate::impl_eq_hash!(Vector4<u32>);
crate::impl_eq_hash!(Vector4<u64>);
crate::impl_eq_hash!(Vector4<u128>);

// Run the macros
crate::setup_add!(Vector4<T>, T);
crate::setup_sub!(Vector4<T>, T);
crate::setup_mul!(Vector4<T>, T);
crate::setup_div!(Vector4<T>, T);
crate::setup_neg!(Vector4<T>, T);
crate::setup_vector_operations!(Vector4<f32>, T, f32);
crate::setup_vector_operations!(Vector4<f64>, T, f64);
crate::impl_elem_wise_comparison!(Vector4<T>, T, Vector4<bool>);

// Dear lord
// I deeply apologize for this
// Floating point to floating point
crate::impl_from_vec4!(Vector4<f64>, f64, f32);
crate::impl_from_vec4!(Vector4<f32>, f32, f64);
// Integers to floating point
crate::impl_from_vec4!(Vector4<f32>, f32, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
crate::impl_from_vec4!(Vector4<f64>, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
// Integers to integers (And floating point to integer)
crate::impl_from_vec4!(Vector4<i8>, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec4!(Vector4<i16>, i16, i8, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec4!(Vector4<i32>, i32, i8, i16, i64, i128, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec4!(Vector4<i64>, i64, i8, i16, i32, i128, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec4!(Vector4<i128>, i128, i8, i16, i32, i64, u8, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec4!(Vector4<u8>, u8, i8, i16, i32, i64, i128, u16, u32, u64, u128, f32, f64);
crate::impl_from_vec4!(Vector4<u16>, u16, i8, i16, i32, i64, i128, u8, u32, u64, u128, f32, f64);
crate::impl_from_vec4!(Vector4<u32>, u32, i8, i16, i32, i64, i128, u8, u16, u64, u128, f32, f64);
crate::impl_from_vec4!(Vector4<u64>, u64, i8, i16, i32, i64, i128, u8, u16, u32, u128, f32, f64);
crate::impl_from_vec4!(Vector4<u128>, u128, i8, i16, i32, i64, i128, u8, u16, u32, u64, f32, f64);
