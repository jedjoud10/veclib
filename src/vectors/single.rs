use crate::{vector::Vector, SupportedValue, VectorElemCount};

impl<T> VectorElemCount for T
    where Self: SupportedValue {
    const ELEM_COUNT: usize = 1;
}


impl<T: SupportedValue> Vector<T> for T {
    fn get_unsized(self) -> crate::vector::UnsizedVector<T>
    where
        T: PartialEq,
    {
        crate::vector::UnsizedVector::Single(self)
    }
}
