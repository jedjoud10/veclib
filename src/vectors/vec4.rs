use super::{Vector2, Vector3};
use crate::{vector::Swizzable};
use std::{hash::Hash, ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign}};

// A simple 4D vector, no simd support what-so-ever
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector4<T> {
    pub data: [T; 4],
}

// Default
impl<T> Default for Vector4<T>
where
    T: Clone + Copy + Sized,
{
    fn default() -> Self {
        Self::ZERO
    }
}

// Implement the vec4 code
#[allow(dead_code)]
impl<T> Vector4<T>
where
    T: Clone + Copy + Sized,
{
    // Constants
    pub const ZERO: Self = Vector4 {
        data: [T::off(), T::off(), T::off(), T::off()],
    };
    pub const X: Self = Vector4 {
        data: [T::on(), T::off(), T::off(), T::off()],
    };
    pub const Y: Self = Vector4 {
        data: [T::off(), T::on(), T::off(), T::off()],
    };
    pub const Z: Self = Vector4 {
        data: [T::off(), T::off(), T::on(), T::off()],
    };
    pub const W: Self = Vector4 {
        data: [T::off(), T::off(), T::off(), T::on()],
    };
    pub const ONE: Self = Vector4 {
        data: [T::on(), T::on(), T::on(), T::on()],
    };
    // Create a new vec4
    pub fn new(f1: T, f2: T, f3: T, f4: T) -> Self {
        Self { data: [f1, f2, f3, f4] }
    }
}

// Indexer
impl<T> Index<usize> for Vector4<T> {
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Vector4<T> {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Swizzle a vec4
impl<T> Swizzable<T> for Vector4<T>
where
    T: Clone + Copy + Sized,
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
impl<T> Vector4<T>
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
    // Get the W coordinate
    pub fn w(&self) -> T {
        self[3]
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
    // Set the W coordinate
    pub fn set_w(&mut self, val: T) {
        self[3] = val;
    }
}

// Element wise comparison
#[allow(dead_code)]
impl<T> Vector4<T>
where
    T: Clone + Copy + Sized + PartialEq + PartialOrd,
{
    // Equals
    pub fn elem_eq(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4<bool> = Vector4::ZERO;
        for i in 0..4 {
            out[i] = self[i] == other[i];
        }
        out
    }
    // Greater then
    pub fn elem_gt(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4<bool> = Vector4::ZERO;
        for i in 0..4 {
            out[i] = self[i] > other[i];
        }
        out
    }
    // Less than
    pub fn elem_lt(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4<bool> = Vector4::ZERO;
        for i in 0..4 {
            out[i] = self[i] < other[i];
        }
        out
    }
    // Greater than or equals
    pub fn elem_gte(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4<bool> = Vector4::ZERO;
        for i in 0..4 {
            out[i] = self[i] >= other[i];
        }
        out
    }
    // Less than or equals
    pub fn elem_lte(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4<bool> = Vector4::ZERO;
        for i in 0..4 {
            out[i] = self[i] <= other[i];
        }
        out
    }
}

// The comparison logic
#[allow(dead_code)]
impl Vector4<bool> {
    // Return true if all the elements are true
    pub fn all(&self) -> bool {
        let mut out: bool = false;
        for i in 0..4 {
            out &= self[i];
        }
        out
    }
    // Return true if one or more elements are true
    pub fn any(&self) -> bool {
        let mut out: bool = false;
        for i in 0..4 {
            out |= self[i];
        }
        out
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
crate::setup_una!(Vector4<T>, T);
crate::setup_vector_arithmatics!(Vector4<f32>, T, f32);
crate::setup_vector_arithmatics!(Vector4<f64>, T, f64);
