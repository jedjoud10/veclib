// Implement the default state trait to a specific integer type
#[macro_export]
macro_rules! impl_default_state {
    ($t:ty) => {
        impl DefaultState for $t {
            const OFF: Self = 0;
            const ON: Self = 1;
        }
    };
}

// Implement the float trait to a specific float type
#[macro_export]
macro_rules! impl_float {
    ($t:ty) => {
        impl Float for $t {
            fn _sqrt(self) -> Self {
                self.sqrt()
            }
        }
    };
}

// Element wise comparison
#[macro_export]
macro_rules! impl_elem_wise_comparison {
    ($t:ty, $a:tt, $out:ty) => {
        // Element wise comparison
        impl<T> $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd,
        {
            // Equals
            pub fn elem_eq(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::ZERO;
                for i in 0..Self::ELEM_COUNT {
                    out[i] = self[i] == other[i];
                }
                out
            }
            // Greater then
            pub fn elem_gt(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::ZERO;
                for i in 0..Self::ELEM_COUNT {
                    out[i] = self[i] > other[i];
                }
                out
            }
            // Less than
            pub fn elem_lt(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::ZERO;
                for i in 0..Self::ELEM_COUNT {
                    out[i] = self[i] < other[i];
                }
                out
            }
            // Greater than or equals
            pub fn elem_gte(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::ZERO;
                for i in 0..Self::ELEM_COUNT {
                    out[i] = self[i] >= other[i];
                }
                out
            }
            // Less than or equals
            pub fn elem_lte(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::ZERO;
                for i in 0..Self::ELEM_COUNT {
                    out[i] = self[i] <= other[i];
                }
                out
            }
        }
        impl $out {
            // Return true if all the elements are true
            pub fn all(&self) -> bool {
                let mut out: bool = true;
                for i in 0..Self::ELEM_COUNT {
                    out &= self[i];
                }
                out
            }
            // Return true if one or more elements are true
            pub fn any(&self) -> bool {
                let mut out: bool = false;
                for i in 0..Self::ELEM_COUNT {
                    out |= self[i];
                }
                out
            }
        }
        impl $out {
            // Select between two vectors using the elements of the current bool vector
            pub fn select<T>(&self, first: &$t, second: &$t) -> $t
            where
                T: Clone + Copy,
            {
                let mut output_vector: $t = first.clone();
                for i in 0..Self::ELEM_COUNT {
                    output_vector[i] = if self[i] { first[i] } else { second[i] }
                }
                return output_vector;
            }
        }
        impl<T> BitAnd for $t
        where
            T: DefaultStates + Clone + Copy + Sized + BitAnd + BitAndAssign,
        {
            type Output = $t;
            // Element wise and
            fn bitand(self, rhs: Self) -> Self::Output {
                let mut out: $t = self;
                for i in 0..Self::ELEM_COUNT {
                    out[i] &= rhs[i];
                }
                return out;
            }
        }
        impl<T> BitOr for $t
        where
            T: DefaultStates + Clone + Copy + Sized + BitOr + BitOrAssign,
        {
            type Output = $t;
            // Element wise or
            fn bitor(self, rhs: Self) -> Self::Output {
                let mut out: $t = self;
                for i in 0..Self::ELEM_COUNT {
                    out[i] |= rhs[i];
                }
                return out;
            }
        }
        impl<T> BitXor for $t
        where
            T: DefaultStates + Clone + Copy + Sized + BitXor + BitXorAssign,
        {
            type Output = $t;
            // Element wise xor
            fn bitxor(self, rhs: Self) -> Self::Output {
                let mut out: $t = self;
                for i in 0..Self::ELEM_COUNT {
                    out[i] ^= rhs[i];
                }
                return out;
            }
        }
        impl<T> Not for $t
        where
            T: DefaultStates + Clone + Copy + Sized + Not<Output = T>,
        {
            type Output = $t;
            // Element wise not
            fn not(self) -> Self::Output {
                let mut out: $t = self;
                for i in 0..Self::ELEM_COUNT {
                    out[i] = !self[i];
                }
                return out;
            }
        }
    };
}
#[macro_export]
macro_rules! impl_hash {
    ($t:ty) => {
        impl Hash for $t {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                for i in 0..<$t>::ELEM_COUNT {
                    self[i].hash(state);
                }
            }
        }
    };
}
