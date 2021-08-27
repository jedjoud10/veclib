use std::ops::Add;

use crate::vectors::*;

// The swizzable trait
pub trait Swizzable<T> {
    // Get vector 4
    fn get4(&self, order: [usize; 4]) -> Vector4<T>;
    // Get vector 3
    fn get3(&self, order: [usize; 3]) -> Vector3<T>;
    // Get vector 2
    fn get2(&self, order: [usize; 2]) -> Vector2<T>;
}

// Trait that has the actual vector functions
pub trait Vectorable<T> {
    // Get the distance from another vector
    fn distance(&self, other: &Self) -> T;
    // Get the length of the current vector
    fn length(&self) -> T;
    // Get the length square of the current vector (Saves us a sqrt operation)
    fn length_sqrt(&self) -> T;
    // Normalize the current vector
    fn normalize(&mut self);
    // Get the normalized value of the current vector without updating it
    fn normalized(&self) -> Self;
    // Get the dot product between two vectors
    fn dot(&self, other: &Self) -> T;
}