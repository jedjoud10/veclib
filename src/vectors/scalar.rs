use crate::{vector::Vector, types::DefaultState};

impl<T> Vector<T> for T
where
    T: DefaultState,
{
    fn get_unsized(self) -> crate::vector::UnsizedVector<T> {
        crate::vector::UnsizedVector::Scalar(self)
    }
}
