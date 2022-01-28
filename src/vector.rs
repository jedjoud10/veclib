use std::{mem::size_of, slice};
use crate::{vectors::*, SupportedValue};

// The swizzable trait
pub trait Swizzable<T> {
    // Get vector 4
    fn get4(&self, order: [usize; 4]) -> Vector4<T>;
    // Get vector 3
    fn get3(&self, order: [usize; 3]) -> Vector3<T>;
    // Get vector 2
    fn get2(&self, order: [usize; 2]) -> Vector2<T>;
}

pub trait VectorElemCount {
    // Get the amount of elements that the self vector has
    const ELEM_COUNT: usize;
}

// The vector trait
pub trait Vector<T> {
    // Turn this into an unsized vector
    fn get_unsized(self) -> UnsizedVector<T>
    where
        T: SupportedValue;
    // Get the pointer of this vector
    fn as_ptr(&self) -> *const T;
    fn as_ptr_mut(&mut self) -> *mut T;
    // Read the bytes of this vector using native endianness
    unsafe fn to_native_bytes(&self) -> &[u8] 
    where 
        Self: Sized
    {
        slice::from_raw_parts(self.as_ptr() as *const u8, size_of::<Self>())
    } 
}

// A vector with interchangeable element count
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnsizedVector<T: SupportedValue> {
    Single(T),
    Vec2(crate::Vector2<T>),
    Vec3(crate::Vector3<T>),
    Vec4(crate::Vector4<T>),
}
