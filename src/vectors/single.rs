use crate::{vector::Vector, SupportedValue};

impl<T: SupportedValue> Vector<T> for T {
    fn get_unsized(self) -> crate::vector::UnsizedVector<T>
    where
        T: PartialEq,
    {
        crate::vector::UnsizedVector::Single(self)
    }
}
