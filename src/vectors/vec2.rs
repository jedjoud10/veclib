use super::{Vector3, Vector4};
use crate::{types::DefaultStates, vector::Swizzable};
use std::{hash::Hash, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign}};

// A simple 2D vector, no simd support what-so-ever
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub data: [T; 2],
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
    // Constants
    pub const ZERO: Self = Vector2 { data: [T::off(), T::off()] };
    pub const X: Self = Vector2 { data: [T::on(), T::on()] };
    pub const Y: Self = Vector2 { data: [T::on(), T::off()] };
    pub const ONE: Self = Vector2 { data: [T::on(), T::on()] };
    // Create a new vec4
    pub fn new(f1: T, f2: T) -> Self {
        Self { data: [f1, f2] }
    }
}

// Indexer
impl<T> Index<usize> for Vector2<T> {
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Vector2<T> {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
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

// Element wise comparison
#[allow(dead_code)]
impl<T> Vector2<T>
where
    T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd,
{
    // Equals
    pub fn elem_eq(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2<bool> = Vector2::ZERO;
        for i in 0..2 {
            out[i] = self[i] == other[i];
        }
        out
    }
    // Greater then
    pub fn elem_gt(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2<bool> = Vector2::ZERO;
        for i in 0..2 {
            out[i] = self[i] > other[i];
        }
        out
    }
    // Less than
    pub fn elem_lt(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2<bool> = Vector2::ZERO;
        for i in 0..2 {
            out[i] = self[i] < other[i];
        }
        out
    }
    // Greater than or equals
    pub fn elem_gte(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2<bool> = Vector2::ZERO;
        for i in 0..2 {
            out[i] = self[i] >= other[i];
        }
        out
    }
    // Less than or equals
    pub fn elem_lte(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2<bool> = Vector2::ZERO;
        for i in 0..2 {
            out[i] = self[i] <= other[i];
        }
        out
    }
}

// Getters
impl<T> Vector2<T>
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
    // Set the X coordinate
    pub fn set_x(&mut self, val: T) {
        self[0] = val;
    }
    // Set the Y coordinate
    pub fn set_y(&mut self, val: T) {
        self[1] = val;
    }
}

// The comparison logic
#[allow(dead_code)]
impl Vector2<bool> {
    // Return true if all the elements are true
    pub fn all(&self) -> bool {
        let mut out: bool = false;
        for i in 0..2 {
            out &= self[i];
        }
        out
    }
    // Return true if one or more elements are true
    pub fn any(&self) -> bool {
        let mut out: bool = false;
        for i in 0..2 {
            out |= self[i];
        }
        out
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
crate::setup_una!(Vector2<T>, T);
crate::setup_vector_arithmatics!(Vector2<f32>, T, f32);
crate::setup_vector_arithmatics!(Vector2<f64>, T, f64);
