use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::{vectors::*};

// The swizzable trait
pub trait Swizzable<T> {
    // Get vector 4
    fn get4(&self, order: [usize; 4]) -> Vector4<T>;
    // Get vector 3
    fn get3(&self, order: [usize; 3]) -> Vector3<T>;
    // Get vector 2
    fn get2(&self, order: [usize; 2]) -> Vector2<T>;
}

pub(crate) trait VectorDefaults {
    // Get the amount of elements that the self vector has
    const ELEM_COUNT: usize;
    // Get the ZERO self value
    const ZERO: Self;
    // Get the ONE self value
    const ONE: Self;
}

// The vector trait
pub(crate) trait Vector<T>
where
    T: num_traits::Num,
{
    // Turn this into an unsized vector
    fn get_unsized(self) -> UnsizedVector<T>;
}

// Some vector arithmetics
pub(crate) trait VectorArithmetics<T>
where
    T: num_traits::Num,
    Self: Sized 
        + VectorDefaults
        + Vector<T>
        + Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Neg
        + Index<usize, Output = T>
        + IndexMut<usize, Output = T>,
{
    // Get the distance from another vector
    fn distance(self, other: Self) -> T {
        let test: Self = self - other;
        return test.length();
    }
    // Get the length square of the current vector (Saves us a sqrt operation)
    fn length_sqrt(self) -> T {
        let mut len: T = T::OFF;
        for i in 0..Self::ELEM_COUNT {
            len += self[i] * self[i];
        }
        return len;
    }
    // Get the length of the current vector
    fn length(self) -> T {
        return self.length_sqrt().sqrt();
    }
    // Normalize the current vector
    fn normalize(&mut self) {
        let len = self.length();
        for i in 0..Self::ELEM_COUNT {
            self[i] /= len;
        }
    }
    // Get the normalized value of the current vector without updating it
    fn normalized(self) -> Self {
        let len = self.length();
        let mut output: Self = Self::ZERO;
        for i in 0..Self::ELEM_COUNT {
            output[i] = self[i] / len;
        }
        return output;
    }
    // Get the dot product between two vectors
    fn dot(self, other: Self) -> T {
        let mut dot: T = 0.0;
        for i in 0..Self::ELEM_COUNT {
            dot += self[i] * other[i];
        }
        return dot;
    }
    // Get the min value between two vec3s
    fn min(self, other: Self) -> Self {
        let mut min = Self::ZERO;
        for i in 0..Self::ELEM_COUNT {
            min[i] = self[i].min(other[i]);
        }
        return min;
    }
    // Get the max value between two vec3s
    fn max(self, other: Self) -> Self {
        let mut min = Self::ZERO;
        for i in 0..Self::ELEM_COUNT {
            min[i] = self[i].max(other[i]);
        }
        return min;
    }
    // Clamp the current value between some bounds and return it
    fn clamp(self, min: Self, max: Self) -> Self {
        return self.min(max).max(min);
    }
    //https://limnu.com/sketch-lerp-unlerp-remap/
    // Lerp between two values using T
    fn lerp(self, other: Self, t: T) -> Self {
        let output = self + ((other - self) * t);
        return output;
    }
}

impl<T> AddAssign for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..Self::ELEM_COUNT {
            self[i] += rhs[i];
        }
    }
}

impl<T> Add for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] += rhs[i];
        }
        self
    }
}

impl<T> Add for &dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] += rhs[i];
        }
        self
    }
}

impl<T> SubAssign for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..Self::ELEM_COUNT {
            self[i] -= rhs[i];
        }
    }
}

impl<T> Sub for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] -= rhs[i];
        }
        self
    }
}

impl<T> Sub for &dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] -= rhs[i];
        }
        self
    }
}

impl<T> MulAssign for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    fn mul_assign(&mut self, rhs: Self) {
        for i in 0..Self::ELEM_COUNT {
            self[i] *= rhs[i];
        }
    }
}

impl<T> Mul for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] *= rhs[i];
        }
        self
    }
}

impl<T> Mul<T> for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] *= rhs;
        }
        self
    }
}

impl<T> Mul for &dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] *= rhs[i];
        }
        self
    }
}

impl<T> Mul<T> for &dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] *= rhs;
        }
        self
    }
}

impl<T> DivAssign for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    fn div_assign(&mut self, rhs: Self) {
        for i in 0..Self::ELEM_COUNT {
            self[i] /= rhs[i];
        }
    }
}

impl<T> Div for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] /= rhs[i];
        }
        self
    }
}

impl<T> Div<T> for dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn div(mut self, rhs: T) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] /= rhs;
        }
        self
    }
}

impl<T> Div for &dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] /= rhs[i];
        }
        self
    }
}

impl<T> Div<T> for &dyn Vector<T>
where
    T: num_traits::Num,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn div(mut self, rhs: T) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] /= rhs;
        }
        self
    }
}

impl<T> Neg for dyn Vector<T>
where
    T: Neg<Output = T>,
    Self: Sized + Index<usize, Output = T> + IndexMut<usize, Output = T> + VectorDefaults,
{
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        for i in 0..Self::ELEM_COUNT {
            self[i] = -self[i];
        }
    }
}

// A vector with interchangeable element count
#[derive(Debug, PartialEq, Eq)]
pub enum UnsizedVector<T>
where
    T: num_traits::Num,
{
    Scalar(T),
    Vec2(crate::Vector2<T>),
    Vec3(crate::Vector3<T>),
    Vec4(crate::Vector4<T>),
}

impl<T> UnsizedVector<T>
where
    T: num_traits::Num,
{
    // Get the amount of elements that each variant has
    pub fn get_elem_count(&self) -> usize {
        match self {
            UnsizedVector::Scalar(_) => 1,
            UnsizedVector::Vec2(_) => 2,
            UnsizedVector::Vec3(_) => 3,
            UnsizedVector::Vec4(_) => 4,
        }
    }
}
