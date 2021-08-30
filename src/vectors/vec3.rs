use super::{Vector2, Vector4};
use crate::{types::DefaultStates, vector::Swizzable};
use std::{
    hash::Hash,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign, BitAnd, BitOr, BitXor, Not},
};

// A simple 3D vector, no simd support what-so-ever
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector3<T> {
    pub data: [T; 3],
}

// Default
impl<T> Default for Vector3<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    fn default() -> Self {
        Self::default_zero()
    }
}

// Implement the vec3 code
#[allow(dead_code)]
impl<T> Vector3<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    // Defaults
    pub fn default_zero() -> Self {
        Vector3 {
            data: [T::off(), T::off(), T::off()],
        }
    }
    pub fn default_x() -> Self {
        Vector3 {
            data: [T::on(), T::off(), T::off()],
        }
    }
    pub fn default_y() -> Self {
        Vector3 {
            data: [T::off(), T::on(), T::off()],
        }
    }
    pub fn default_z() -> Self {
        Vector3 {
            data: [T::off(), T::off(), T::on()],
        }
    }
    pub fn default_one() -> Self {
        Vector3 {
            data: [T::on(), T::on(), T::on()],
        }
    }
    // Create a new vec4
    pub fn new(f1: T, f2: T, f3: T) -> Self {
        Self { data: [f1, f2, f3] }
    }
}

// Indexer
impl<T> Index<usize> for Vector3<T> {
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Vector3<T> {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Swizzle a vec3
impl<T> Swizzable<T> for Vector3<T>
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

// Getters and setters
impl<T> Vector3<T>
where
    T: Copy,
{
    // Get the X coordinate
    pub fn x(&self) -> T {
        self[0]
    }
    // Get the Y coordinate
    pub fn y(&self) -> T {
        self[1]
    }
    // Get the Z coordinate
    pub fn z(&self) -> T {
        self[2]
    }
    // Set the X coordinate
    pub fn set_x(&mut self, val: T) {
        self[0] = val;
    }
    // Set the Y coordinate
    pub fn set_y(&mut self, val: T) {
        self[1] = val;
    }
    // Set the Z coordinate
    pub fn set_z(&mut self, val: T) {
        self[2] = val;
    }
}

// Eq and Hash for int types
crate::impl_eq_hash!(Vector3<i16>);
crate::impl_eq_hash!(Vector3<i32>);
crate::impl_eq_hash!(Vector3<i64>);
crate::impl_eq_hash!(Vector3<i128>);
crate::impl_eq_hash!(Vector3<u16>);
crate::impl_eq_hash!(Vector3<u32>);
crate::impl_eq_hash!(Vector3<u64>);
crate::impl_eq_hash!(Vector3<u128>);

// Run the macros
crate::setup_add!(Vector3<T>, T);
crate::setup_sub!(Vector3<T>, T);
crate::setup_mul!(Vector3<T>, T);
crate::setup_div!(Vector3<T>, T);
crate::setup_una!(Vector3<T>, T);
crate::setup_vector_arithmatics!(Vector3<f32>, T, f32);
crate::setup_vector_arithmatics!(Vector3<f64>, T, f64);
crate::impl_elem_wise_comparison!(Vector3<T>, T, Vector3<bool>);

// Dear lord
// I deeply apologize for this
// Floating point to floating point
crate::impl_from_vec3!(Vector3<f64>, f64, f32);
crate::impl_from_vec3!(Vector3<f32>, f32, f64);
// Integers to floating point
crate::impl_from_vec3!(Vector3<f32>, f32, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
crate::impl_from_vec3!(Vector3<f64>, f64, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
// Integers to integers
crate::impl_from_vec3!(Vector3<i8>, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
crate::impl_from_vec3!(Vector3<i16>, i16, i8, i32, i64, i128, u8, u16, u32, u64, u128);
crate::impl_from_vec3!(Vector3<i32>, i32, i8, i16, i64, i128, u8, u16, u32, u64, u128);
crate::impl_from_vec3!(Vector3<i64>, i64, i8, i16, i32, i128, u8, u16, u32, u64, u128);
crate::impl_from_vec3!(Vector3<i128>, i128, i8, i16, i32, i64, u8, u16, u32, u64, u128);
crate::impl_from_vec3!(Vector3<u8>, u8, i8, i16, i32, i64, i128, u16, u32, u64, u128);
crate::impl_from_vec3!(Vector3<u16>, u16, i8, i16, i32, i64, i128, u8, u32, u64, u128);
crate::impl_from_vec3!(Vector3<u32>, u32, i8, i16, i32, i64, i128, u8, u16, u64, u128);
crate::impl_from_vec3!(Vector3<u64>, u64, i8, i16, i32, i64, i128, u8, u16, u32, u128);
crate::impl_from_vec3!(Vector3<u128>, u128, i8, i16, i32, i64, i128, u8, u16, u32, u64);

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
