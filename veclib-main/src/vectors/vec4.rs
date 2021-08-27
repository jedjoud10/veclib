use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use crate::{types::DefaultStates, vector::Swizzable};
use super::{Vector2, Vector3};

// A simple 4D vector, no simd support what-so-ever
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector4<T> {
    pub data: [T; 4],
}

// Implement the vec4 code
#[allow(dead_code)]
impl<T> Vector4<T> where T: DefaultStates + Clone + Copy + Sized {
    // Constants
    pub const ZERO: Self = Vector4 { data: [T::off(), T::off(), T::off(), T::off()] };
    pub const X: Self = Vector4 { data: [T::on(), T::off(), T::off(), T::off()] };
    pub const Y: Self = Vector4 { data: [T::off(), T::on(), T::off(), T::off()] };
    pub const Z: Self = Vector4 { data: [T::off(), T::off(), T::on(), T::off()] };
    pub const W: Self = Vector4 { data: [T::off(), T::off(), T::off(), T::on()] };
    pub const ONE: Self = Vector4 { data: [T::on(), T::on(), T::on(), T::on()] };
    // Create a new vec4
    pub fn new(f1: T, f2: T, f3: T, f4: T) -> Self {
        return Self { data: [f1, f2, f3, f4] }
    }
}

// Indexer
impl<T> Index<usize> for Vector4<T> {
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[index];
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Vector4<T> {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.data[index];
    }
}

// Swizzle a vec4
impl<T> Swizzable<T> for Vector4<T> where T: DefaultStates + Clone + Copy + Sized {
    fn get4(&self, order: [usize; 4]) -> Vector4<T> {
        return Vector4::new(self[order[0]], self[order[1]], self[order[2]], self[order[3]]);
    }

    fn get3(&self, order: [usize; 3]) -> Vector3<T> {
        return Vector3::new(self[order[0]], self[order[1]], self[order[2]]);
    }

    fn get2(&self, order: [usize; 2]) -> Vector2<T> {
        return Vector2::new(self[order[0]], self[order[1]]);
    }
}

// Getters
impl<T> Vector4<T> where T: Copy {
    // Get the X coordinate
    pub fn x(&self) -> T { return self[0]; }
    // Get the Y coordinate
    pub fn y(&self) -> T { return self[1]; } 
    // Get the Z coordinate
    pub fn z(&self) -> T { return self[2]; } 
    // Get the W coordinate
    pub fn w(&self) -> T { return self[3]; } 
}

// Element wise comparison 
#[allow(dead_code)]
impl<T> Vector4<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd {
    // Equals
    pub fn elem_eq(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4::<bool> = Vector4::ZERO;
        for i in 0..4 { out[i] = self[i] == other[i]; } 
        return out;
    }
    // Greater then
    pub fn elem_gt(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4::<bool> = Vector4::ZERO;
        for i in 0..4 { out[i] = self[i] > other[i]; } 
        return out;
    }
    // Less than
    pub fn elem_lt(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4::<bool> = Vector4::ZERO;
        for i in 0..4 { out[i] = self[i] < other[i]; } 
        return out;
    }
    // Greater than or equals
    pub fn elem_gte(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4::<bool> = Vector4::ZERO;
        for i in 0..4 { out[i] = self[i] >= other[i]; } 
        return out;
    }
    // Less than or equals
    pub fn elem_lte(&self, other: &Self) -> Vector4<bool> {
        let mut out: Vector4::<bool> = Vector4::ZERO;
        for i in 0..4 { out[i] = self[i] <= other[i]; } 
        return out;
    }
}

// The comparison logic
#[allow(dead_code)]
impl Vector4<bool> {
    // Return true if all the elements are true
    pub fn all(&self) -> bool {
        let mut out: bool = false;
        for i in 0..4 { out &= self[i]; } 
        return out;
    }
    // Return true if one or more elements are true
    pub fn any(&self) -> bool {
        let mut out: bool = false;
        for i in 0..4 { out |= self[i]; } 
        return out;
    }
}

// Run the procedural macros to setup all the operators
crate::setup_addition!(4);
crate::setup_subtraction!(4);
crate::setup_multiplication!(4);
crate::setup_division!(4);
// Setup the shared vector arithmatics
crate::setup_vector_maths_f32!(4);