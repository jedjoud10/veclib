// Implement the default state struct to a specific integer type
#[macro_export]
macro_rules! impl_default_state {
    ($t:ty) => {
        impl DefaultStates for $t {
            const OFF: Self = 0;
            const ON: Self = 1;
        }
    };
}

#[macro_export]
macro_rules! setup_neg {
    ($t:ty, $a:tt) => {
        impl<T> Neg for $t
        where
            T: DefaultStates + Neg<Output = T> + Copy,
        {
            type Output = $t;

            fn neg(mut self) -> Self::Output {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = -self[i];
                }
                return self;
            }
        }
        impl<T> Neg for &$t
        where
            T: DefaultStates + Neg<Output = T> + Copy,
        {
            type Output = $t;

            fn neg(self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..Self::ELEM_COUNT {
                    output[i] = -self[i];
                }
                return output;
            }
        }
    };
}

#[macro_export]
macro_rules! setup_vector_arithmatics {
    ($t:ty, $a:tt, $f: ty) => {
        // Setup the shared vector arithmatics
        impl $t {
            // Get the distance from another vector
            pub fn distance(self, other: Self) -> $f {
                let test: $t = self - other;
                return test.length();
            }
            // Get the length square of the current vector (Saves us a sqrt operation)
            pub fn length_sqrt(self) -> $f {
                let mut len: $f = 0.0;
                for i in 0..Self::ELEM_COUNT {
                    len += self[i] * self[i];
                }
                return len;
            }
            // Get the length of the current vector
            pub fn length(self) -> $f {
                return self.length_sqrt().sqrt();
            }
            // Normalize the current vector
            pub fn normalize(&mut self) {
                let len = self.length();
                for i in 0..Self::ELEM_COUNT {
                    self[i] /= len;
                }
            }
            // Get the normalized value of the current vector without updating it
            pub fn normalized(self) -> Self {
                let len = self.length();
                let mut output: Self = Self::ZERO;
                for i in 0..Self::ELEM_COUNT {
                    output[i] = self[i] / len;
                }
                return output;
            }
            // Get the dot product between two vectors
            pub fn dot(self, other: Self) -> $f {
                let mut dot: $f = 0.0;
                for i in 0..Self::ELEM_COUNT {
                    dot += self[i] * other[i];
                }
                return dot;
            }
            // Get the min value between two vec3s
            pub fn min(self, other: Self) -> Self {
                let mut min = <$t>::ZERO;
                for i in 0..Self::ELEM_COUNT {
                    min[i] = self[i].min(other[i]);
                }
                return min;
            }
            // Get the max value between two vec3s
            pub fn max(self, other: Self) -> Self {
                let mut min = <$t>::ZERO;
                for i in 0..Self::ELEM_COUNT {
                    min[i] = self[i].max(other[i]);
                }
                return min;
            }
            // Clamp the current value between some bounds and return it
            pub fn clamp(self, min: Self, max: Self) -> Self {
                return self.min(max).max(min);
            }
            //https://limnu.com/sketch-lerp-unlerp-remap/
            // Lerp between two values using T
            pub fn lerp(self, other: Self, t: $f) -> Self {
                let output = (self + ((other - self) * t));
                return output;
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
