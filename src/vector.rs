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

pub(crate) trait VectorElemCount {
    // Get the amount of elements that the self vector has
    const ELEM_COUNT: usize;
}

// The vector trait
pub trait Vector<T> {
    // Turn this into an unsized vector
    fn get_unsized(self) -> UnsizedVector<T>
    where
        T: PartialEq + SupportedValue;
}

// A vector with interchangeable element count
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnsizedVector<T: SupportedValue + PartialEq> {
    Single(T),
    Vec2(crate::Vector2<T>),
    Vec3(crate::Vector3<T>),
    Vec4(crate::Vector4<T>),
}