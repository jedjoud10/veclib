use super::{Vector2, Vector4};
use crate::{types::DefaultStates, vector::Swizzable};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

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
        Self::ZERO
    }
}

// Implement the vec3 code
#[allow(dead_code)]
impl<T> Vector3<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    // Constants
    pub const ZERO: Self = Vector3 {
        data: [T::off(), T::off(), T::off()],
    };
    pub const X: Self = Vector3 {
        data: [T::on(), T::off(), T::off()],
    };
    pub const Y: Self = Vector3 {
        data: [T::off(), T::on(), T::off()],
    };
    pub const Z: Self = Vector3 {
        data: [T::off(), T::off(), T::on()],
    };
    pub const ONE: Self = Vector3 {
        data: [T::on(), T::on(), T::on()],
    };
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

// Element wise comparison
#[allow(dead_code)]
impl<T> Vector3<T>
where
    T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd,
{
    // Equals
    pub fn elem_eq(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3<bool> = Vector3::ZERO;
        for i in 0..3 {
            out[i] = self[i] == other[i];
        }
        out
    }
    // Greater then
    pub fn elem_gt(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3<bool> = Vector3::ZERO;
        for i in 0..3 {
            out[i] = self[i] > other[i];
        }
        out
    }
    // Less than
    pub fn elem_lt(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3<bool> = Vector3::ZERO;
        for i in 0..3 {
            out[i] = self[i] < other[i];
        }
        out
    }
    // Greater than or equals
    pub fn elem_gte(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3<bool> = Vector3::ZERO;
        for i in 0..3 {
            out[i] = self[i] >= other[i];
        }
        out
    }
    // Less than or equals
    pub fn elem_lte(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3<bool> = Vector3::ZERO;
        for i in 0..3 {
            out[i] = self[i] <= other[i];
        }
        out
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

// The comparison logic
#[allow(dead_code)]
impl Vector3<bool> {
    // Return true if all the elements are true
    pub fn all(&self) -> bool {
        let mut out: bool = false;
        for i in 0..3 {
            out &= self[i];
        }
        out
    }
    // Return true if one or more elements are true
    pub fn any(&self) -> bool {
        let mut out: bool = false;
        for i in 0..3 {
            out |= self[i];
        }
        out
    }
}

// Run the macros
crate::setup_add!(Vector3<T>, T);
crate::setup_sub!(Vector3<T>, T);
crate::setup_mul!(Vector3<T>, T);
crate::setup_div!(Vector3<T>, T);
crate::setup_una!(Vector3<T>, T);
crate::setup_vector_arithmatics!(Vector3<f32>, T, f32);
crate::setup_vector_arithmatics!(Vector3<f64>, T, f64);

// Vector3 arithmatics
impl Vector3<f32> {
    // Get the cross product between two vectors
    pub fn cross(self, other: Self) -> Vector3<f32> {
        // Normalize both self and other
        let a = self.normalized();
        let b = other.normalized();
        Vector3::new(a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[1] * b[0] - a[0] * b[1])
    }
}
impl Vector3<f64> {
    // Get the cross product between two vectors
    pub fn cross(self, other: Self) -> Vector3<f64> {
        // Normalize both self and other
        let a = self.normalized();
        let b = other.normalized();
        Vector3::new(a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[1] * b[0] - a[0] * b[1])
    }
}
