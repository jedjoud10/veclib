use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use crate::{types::DefaultStates, vector::Swizzable};
use super::{Vector2, Vector4};

// A simple 3D vector, no simd support what-so-ever
#[derive(PartialEq, Debug)]
pub struct Vector3<T> {
    pub data: [T; 3],
}

// Implement the vec3 code
#[allow(dead_code)]
impl<T> Vector3<T> where T: DefaultStates + Clone + Copy + Sized {
    // Constants
    pub const ZERO: Self = Vector3 { data: [T::off(), T::off(), T::off()] };
    pub const X: Self = Vector3 { data: [T::on(), T::off(), T::off()] };
    pub const Y: Self = Vector3 { data: [T::off(), T::on(), T::off()] };
    pub const Z: Self = Vector3 { data: [T::off(), T::off(), T::on()] };
    pub const ONE: Self = Vector3 { data: [T::on(), T::on(), T::on()] };
    // Create a new vec4
    pub fn new(f1: T, f2: T, f3: T) -> Self {
        return Self { data: [f1, f2, f3] }
    }
}

// Indexer
impl<T> Index<usize> for Vector3<T> {
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[index];
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Vector3<T> {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.data[index];
    }
}

// Swizzle a vec3
impl<T> Swizzable<T> for Vector3<T> where T: DefaultStates + Clone + Copy + Sized {
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
impl<T> Vector3<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd {
    // Equals
    pub fn elem_eq(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3::<bool> = Vector3::ZERO;
        for i in 0..3 { out[i] = self[i] == other[i]; } 
        return out;
    }
    // Greater then
    pub fn elem_gt(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3::<bool> = Vector3::ZERO;
        for i in 0..3 { out[i] = self[i] > other[i]; } 
        return out;
    }
    // Less than
    pub fn elem_lt(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3::<bool> = Vector3::ZERO;
        for i in 0..3 { out[i] = self[i] < other[i]; } 
        return out;
    }
    // Greater than or equals
    pub fn elem_gte(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3::<bool> = Vector3::ZERO;
        for i in 0..3 { out[i] = self[i] >= other[i]; } 
        return out;
    }
    // Less than or equals
    pub fn elem_lte(&self, other: &Self) -> Vector3<bool> {
        let mut out: Vector3::<bool> = Vector3::ZERO;
        for i in 0..3 { out[i] = self[i] <= other[i]; } 
        return out;
    }
}

// The comparison logic
#[allow(dead_code)]
impl Vector3<bool> {
    // Return true if all the elements are true
    pub fn all(&self) -> bool {
        let mut out: bool = false;
        for i in 0..3 { out &= self[i]; } 
        return out;
    }
    // Return true if one or more elements are true
    pub fn any(&self) -> bool {
        let mut out: bool = false;
        for i in 0..3 { out |= self[i]; } 
        return out;
    }
}

// The math operations that will be applied to the vectors
impl<T> Add for Vector3<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
    type Output = Vector3<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 { self[i] = self[i] + rhs[i]; }
        return self;
    }
}
impl<T> AddAssign for Vector3<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..3 { self[i] = self[i] + rhs[i]; }       
    }
}
impl<T> Sub for Vector3<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
    type Output = Vector3<T>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 { self[i] = self[i] - rhs[i]; }
        return self;
    }
}
impl<T> SubAssign for Vector3<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..3 { self[i] = self[i] - rhs[i]; }       
    }
}
impl<T> Mul for Vector3<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
    type Output = Vector3<T>;

    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 { self[i] = self[i] * rhs[i]; }
        return self;
    }
}
impl<T> MulAssign for Vector3<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..3 { self[i] = self[i] * rhs[i]; }       
    }
}
impl<T> Div for Vector3<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
    type Output = Vector3<T>;

    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..3 { self[i] = self[i] / rhs[i]; }
        return self;
    }
}
impl<T> DivAssign for Vector3<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..3 { self[i] = self[i] * rhs[i]; }       
    }
}