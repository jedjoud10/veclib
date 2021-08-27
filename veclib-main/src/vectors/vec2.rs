use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Index, IndexMut};
use crate::{types::DefaultStates, vector::{Swizzable, Vectorable}};
use super::{Vector3, Vector4};

// A simple 2D vector, no simd support what-so-ever
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector2<T> {
    pub data: [T; 2],
}

// Implement the vec3 code
#[allow(dead_code)]
impl<T> Vector2<T> where T: DefaultStates + Clone + Copy + Sized {
    // Constants
    pub const ZERO: Self = Vector2 { data: [T::off(), T::off()] };
    pub const X: Self = Vector2 { data: [T::on(), T::on()] };
    pub const Y: Self = Vector2 { data: [T::on(), T::off()] };
    pub const ONE: Self = Vector2 { data: [T::on(), T::on()] };
    // Create a new vec4
    pub fn new(f1: T, f2: T) -> Self {
        return Self { data: [f1, f2] }
    }
}

// Indexer
impl<T> Index<usize> for Vector2<T> {
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[index];
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Vector2<T> {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.data[index];
    }
}

// Swizzle a vec2
impl<T> Swizzable<T> for Vector2<T> where T: DefaultStates + Clone + Copy + Sized {
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

// Element wise comparison 
#[allow(dead_code)]
impl<T> Vector2<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd {
    // Equals
    pub fn elem_eq(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2::<bool> = Vector2::ZERO;
        for i in 0..2 { out[i] = self[i] == other[i]; } 
        return out;
    }
    // Greater then
    pub fn elem_gt(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2::<bool> = Vector2::ZERO;
        for i in 0..2 { out[i] = self[i] > other[i]; } 
        return out;
    }
    // Less than
    pub fn elem_lt(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2::<bool> = Vector2::ZERO;
        for i in 0..2 { out[i] = self[i] < other[i]; } 
        return out;
    }
    // Greater than or equals
    pub fn elem_gte(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2::<bool> = Vector2::ZERO;
        for i in 0..2 { out[i] = self[i] >= other[i]; } 
        return out;
    }
    // Less than or equals
    pub fn elem_lte(&self, other: &Self) -> Vector2<bool> {
        let mut out: Vector2::<bool> = Vector2::ZERO;
        for i in 0..2 { out[i] = self[i] <= other[i]; } 
        return out;
    }
}

// Getters
impl<T> Vector2<T> where T: Copy {
    // Get the X coordinate
    pub fn x(&self) -> T { return self[0]; }
    // Get the Y coordinate
    pub fn y(&self) -> T { return self[1]; } 
}

// The comparison logic
#[allow(dead_code)]
impl Vector2<bool> {
    // Return true if all the elements are true
    pub fn all(&self) -> bool {
        let mut out: bool = false;
        for i in 0..2 { out &= self[i]; } 
        return out;
    }
    // Return true if one or more elements are true
    pub fn any(&self) -> bool {
        let mut out: bool = false;
        for i in 0..2 { out |= self[i]; } 
        return out;
    }
}

// Run the procedural macros to setup all the operators
crate::setup_addition!(2);
crate::setup_subtraction!(2);
crate::setup_multiplication!(2);
crate::setup_division!(2);
// Setup the shared vector arithmatics
crate::setup_vector_maths_f32!(2);