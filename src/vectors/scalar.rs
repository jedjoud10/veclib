use crate::{vector::Vector, SupportedValue};

impl<T> Vector<T> for T
where
    T: SupportedValue,
{
    fn get_unsized(self) -> crate::vector::UnsizedVector<T>
    where
        T: PartialEq,
    {
        crate::vector::UnsizedVector::Scalar(self)
    }
}
