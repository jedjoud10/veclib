use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, DivAssign, Div, Neg, Rem, RemAssign};

use num::{One, Zero, Float, traits::{NumAssign, NumAssignOps}};
// A simple 2D vector
#[derive(Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Vector2<T> {
    pub(crate) inner: [T; 2]
}

impl<Real: num::Float + NumAssign + NumAssignOps> Vector2<Real> {
    // Get the distance from another vector
    pub fn distance(self, other: Self) -> Real {
        let test: Self = self - other;
        return test.length();
    }
    // Get the length square of the current vector (Saves us a sqrt operation)
    pub fn length_sqrt(self) -> Real {
        let mut len: Real = Zero::zero();
        for val in self.inner {
            len += val.powi(2);
        }
        return len;
    }
    // Get the length of the current vector
    pub fn length(self) -> Real {
        return self.length_sqrt().sqrt();
    }
    // Normalize the current vector
    pub fn normalize(&mut self) {
        let len = self.length();
        *self /= len;
    }
    // Get the dot product between two vectors
    pub fn dot(self, other: Self) -> Real {
        let mut dot: Real = Zero::zero();
        for val in (self - other).inner.into_iter() {
            dot += val
        }
        dot
    }
    // Floor
    pub fn floor(mut self) -> Self {
        for val in self.inner.iter_mut() {
            *val = val.floor();
        }
        self
    }
    // Round
    pub fn round(mut self) -> Self {
        for val in self.inner.iter_mut() {
            *val = val.round();
        }
        self
    }
    // Ceil  
    pub fn ceil(mut self) -> Self {
        for val in self.inner.iter_mut() {
            *val = val.ceil();
        }
        self
    }        
    //https://limnu.com/sketch-lerp-unlerp-remap/
    // Lerp between two values using T
    pub fn lerp(self, other: Self, t: Real) -> Self {
        let output = self + ((other - self) * t);
        return output;
    }
}


// Operators
impl<T: AddAssign + Copy> Add for Vector2<T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        Self {
            inner: {
                for (i, val) in self.inner.iter_mut().enumerate() {
                    *val += rhs.inner[i];
                }
                self.inner
            }
        }
    }
}
impl<T: AddAssign + Copy> Add<T> for Vector2<T> {
    type Output = Self;
    fn add(mut self, rhs: T) -> Self::Output {
        Self {
            inner: {
                for val in self.inner.iter_mut() {
                    *val += rhs;
                }
                self.inner
            }
        }
    }
}
impl<T: AddAssign + Copy> Add<&T> for Vector2<T> {
    type Output = Self;
    fn add(mut self, rhs: &T) -> Self::Output {
        Self {
            inner: {
                for val in self.inner.iter_mut() {
                    *val += *rhs;
                }
                self.inner
            }
        }
    }
}
impl<T: AddAssign + Copy> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        for (i, val) in self.inner.iter_mut().enumerate() {
            *val += rhs.inner[i];
        }
    }
}
impl<T: AddAssign + Copy> AddAssign<T> for Vector2<T> {
    fn add_assign(&mut self, rhs: T) {
        for val in self.inner.iter_mut() {
            *val += rhs;
        }
    }
}
impl<T: AddAssign + Copy> AddAssign<&T> for Vector2<T> {
    fn add_assign(&mut self, rhs: &T) {
        for val in self.inner.iter_mut() {
            *val += *rhs;
        }
    }
}
impl<T: SubAssign + Copy> Sub for Vector2<T> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        Self {
            inner: {
                for (i, val) in self.inner.iter_mut().enumerate() {
                    *val -= rhs.inner[i];
                }
                self.inner
            }
        }
    }
}
impl<T: SubAssign + Copy> Sub<T> for Vector2<T> {
    type Output = Self;
    fn sub(mut self, rhs: T) -> Self::Output {
        Self {
            inner: {
                for val in self.inner.iter_mut() {
                    *val -= rhs;
                }
                self.inner
            }
        }
    }
}
impl<T: SubAssign + Copy> Sub<&T> for Vector2<T> {
    type Output = Self;
    fn sub(mut self, rhs: &T) -> Self::Output {
        Self {
            inner: {
                for val in self.inner.iter_mut() {
                    *val -= *rhs;
                }
                self.inner
            }
        }
    }
}
impl<T: SubAssign + Copy> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        for (i, val) in self.inner.iter_mut().enumerate() {
            *val -= rhs.inner[i];
        }
    }
}
impl<T: SubAssign + Copy> SubAssign<T> for Vector2<T> {
    fn sub_assign(&mut self, rhs: T) {
        for val in self.inner.iter_mut() {
            *val -= rhs;
        }
    }
}
impl<T: SubAssign + Copy> SubAssign<&T> for Vector2<T> {
    fn sub_assign(&mut self, rhs: &T) {
        for val in self.inner.iter_mut() {
            *val -= *rhs;
        }
    }
}
impl<T: MulAssign + Copy> Mul for Vector2<T> {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        Self {
            inner: {
                for (i, val) in self.inner.iter_mut().enumerate() {
                    *val *= rhs.inner[i];
                }
                self.inner
            }
        }
    }
}
impl<T: MulAssign + Copy> Mul<T> for Vector2<T> {
    type Output = Self;
    fn mul(mut self, rhs: T) -> Self::Output {
        Self {
            inner: {
                for val in self.inner.iter_mut() {
                    *val *= rhs;
                }
                self.inner
            }
        }
    }
}
impl<T: MulAssign + Copy> Mul<&T> for Vector2<T> {
    type Output = Self;
    fn mul(mut self, rhs: &T) -> Self::Output {
        Self {
            inner: {
                for val in self.inner.iter_mut() {
                    *val *= *rhs;
                }
                self.inner
            }
        }
    }
}
impl<T: MulAssign + Copy> MulAssign for Vector2<T> {
    fn mul_assign(&mut self, rhs: Self) {
        for (i, val) in self.inner.iter_mut().enumerate() {
            *val *= rhs.inner[i];
        }
    }
}
impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        for val in self.inner.iter_mut() {
            *val *= rhs;
        }
    }
}
impl<T: MulAssign + Copy> MulAssign<&T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: &T) {
        for val in self.inner.iter_mut() {
            *val *= *rhs;
        }
    }
}
impl<T: DivAssign + Copy> Div for Vector2<T> {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        Self {
            inner: {
                for (i, val) in self.inner.iter_mut().enumerate() {
                    *val /= rhs.inner[i];
                }
                self.inner
            }
        }
    }
}
impl<T: DivAssign + Copy> Div<T> for Vector2<T> {
    type Output = Self;
    fn div(mut self, rhs: T) -> Self::Output {
        Self {
            inner: {
                for val in self.inner.iter_mut() {
                    *val /= rhs;
                }
                self.inner
            }
        }
    }
}
impl<T: DivAssign + Copy> Div<&T> for Vector2<T> {
    type Output = Self;
    fn div(mut self, rhs: &T) -> Self::Output {
        Self {
            inner: {
                for val in self.inner.iter_mut() {
                    *val /= *rhs;
                }
                self.inner
            }
        }
    }
}
impl<T: DivAssign + Copy> DivAssign for Vector2<T> {
    fn div_assign(&mut self, rhs: Self) {
        for (i, val) in self.inner.iter_mut().enumerate() {
            *val /= rhs.inner[i];
        }
    }
}
impl<T: DivAssign + Copy> DivAssign<T> for Vector2<T> {
    fn div_assign(&mut self, rhs: T) {
        for val in self.inner.iter_mut() {
            *val /= rhs;
        }
    }
}
impl<T: DivAssign + Copy> DivAssign<&T> for Vector2<T> {
    fn div_assign(&mut self, rhs: &T) {
        for val in self.inner.iter_mut() {
            *val /= *rhs;
        }
    }
}
impl<T: Neg<Output = T> + Copy> Neg for Vector2<T> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for val in self.inner.iter_mut() { 
            *val = val.neg();
        }
        self
    }
}
impl<T: Rem<Output = T> + Copy> Rem for Vector2<T> {
    type Output = Vector2<T>;

    fn rem(mut self, rhs: Self) -> Self::Output {
        Self {
            inner: {
                for (i, val) in self.inner.iter_mut().enumerate() { 
                    *val = *val % rhs.inner[i];
                } 
                self.inner
            }
        }
    }
}
impl<T: Rem<Output = T> + Copy> Rem<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn rem(mut self, rhs: T) -> Self::Output {
        Self {
            inner: {
                for val in self.inner.iter_mut() { 
                    *val = *val % rhs;
                } 
                self.inner
            }
        }
    }
}
impl<T: Rem<Output = T> + Copy> RemAssign for Vector2<T> {
    fn rem_assign(&mut self, rhs: Self) {
        for (i, val) in self.inner.iter_mut().enumerate() { 
            *val = *val % rhs.inner[i];
        } 
    }
}
impl<T: Rem<Output = T> + Copy> RemAssign<T> for Vector2<T> {
    fn rem_assign(&mut self, rhs: T) {
        for val in self.inner.iter_mut() { 
            *val = *val % rhs;
        } 
    }
}
impl<T: Rem<Output = T> + Copy> RemAssign<&T> for Vector2<T> {
    fn rem_assign(&mut self, rhs: &T) {
        for val in self.inner.iter_mut() { 
            *val = *val % *rhs;
        } 
    }
}
impl<T: Zero + AddAssign + Copy> Zero for Vector2<T> {
    fn zero() -> Self {
        Self {
            inner: [T::zero(), T::zero()]
        }
    }

    fn is_zero(&self) -> bool {
        self.inner[0].is_zero() && self.inner[1].is_zero()
    }
}
impl<T: One + MulAssign + Copy> One for Vector2<T> {
    fn one() -> Self {
        Self {
            inner: [T::one(), T::one()]
        }
    }
}