use crate::{vector::Vector};

impl<T> Vector<T> for T
where
    T: num_traits::Num,
{
    fn get_unsized(self) -> crate::vector::UnsizedVector<T> {
        crate::vector::UnsizedVector::Scalar(self)
    }
}
