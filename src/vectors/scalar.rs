use crate::{vector::Vector, DefaultStates};

impl<T> Vector<T> for T
where
    T: DefaultStates + Clone + Copy,
{
    fn get_unsized(self) -> crate::vector::UnsizedVector<T> {
        crate::vector::UnsizedVector::Scalar(self)
    }
}
