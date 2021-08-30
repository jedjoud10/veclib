// Macro for turning a specific type into another, only for vec2s
#[macro_export]
macro_rules! impl_from_vec2 {
    ($t:ty, $a:ty, $($f:ty),+) => {
        $(
            impl From<Vector2<$f>> for $t {
                fn from(val: Vector2<$f>) -> Self {
                    <$t>::new(val[0] as $a, val[1] as $a)
                }
            }
        )*
    };
}
// Macro for turning a specific type into another, only for vec3s
#[macro_export]
macro_rules! impl_from_vec3 {
    ($t:ty, $a:ty, $($f:ty),+) => {
        $(
            impl From<Vector3<$f>> for $t {
                fn from(val: Vector3<$f>) -> Self {
                    <$t>::new(val[0] as $a, val[1] as $a, val[2] as $a)
                }
            }
        )*
    };
}
// Macro for turning a specific type into another, only for vec4s
#[macro_export]
macro_rules! impl_from_vec4 {
    ($t:ty, $a:ty, $($f:ty),+) => {
        $(
            impl From<Vector4<$f>> for $t {
                fn from(val: Vector4<$f>) -> Self {
                    <$t>::new(val[0] as $a, val[1] as $a, val[2] as $a, val[3] as $a)
                }
            }
        )*
    };
}

// Implement the default state struct to a specific integer type
#[macro_export]
macro_rules! impl_default_state {
    ($t:ty) => {
        impl DefaultStates for $t {
            fn off() -> Self {
                0
            }
            fn on() -> Self {
                1
            }
        }
    };
}

#[macro_export]
macro_rules! setup_add {
    ($t:ty, $a:tt) => {
        impl<T> Add for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T>,
        {
            type Output = $t;

            fn add(mut self, rhs: Self) -> Self::Output {
                for i in 0..self.data.len() {
                    self[i] = self[i] + rhs[i];
                }
                return self;
            }
        }
        /*
        impl<T> Add for &$t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
            type Output = $t;

            fn add(self, rhs: Self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..self.data.len() { output[i] = self[i] + rhs[i]; }
                return output;
            }
        }
        */
        impl<T> AddAssign for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T>,
        {
            fn add_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() {
                    self[i] = self[i] + rhs[i];
                }
            }
        }
        impl<T> Add<T> for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T>,
        {
            type Output = $t;

            fn add(mut self, rhs: T) -> Self::Output {
                for i in 0..self.data.len() {
                    self[i] = self[i] + rhs;
                }
                return self;
            }
        }
        impl<T> AddAssign<T> for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T>,
        {
            fn add_assign(&mut self, rhs: T) {
                for i in 0..self.data.len() {
                    self[i] = self[i] + rhs;
                }
            }
        }
    };
}

#[macro_export]
macro_rules! setup_sub {
    ($t:ty, $a:tt) => {
        impl<T> Sub for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T>,
        {
            type Output = $t;

            fn sub(mut self, rhs: Self) -> Self::Output {
                for i in 0..self.data.len() {
                    self[i] = self[i] - rhs[i];
                }
                return self;
            }
        }
        /*
        impl<T> Sub for &$t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
            type Output = $t;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..self.data.len() { output[i] = self[i] - rhs[i]; }
                return output;
            }
        }
        */
        impl<T> SubAssign for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T>,
        {
            fn sub_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() {
                    self[i] = self[i] - rhs[i];
                }
            }
        }
        impl<T> Sub<T> for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T>,
        {
            type Output = $t;

            fn sub(mut self, rhs: T) -> Self::Output {
                for i in 0..self.data.len() {
                    self[i] = self[i] - rhs;
                }
                return self;
            }
        }
        impl<T> SubAssign<T> for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T>,
        {
            fn sub_assign(&mut self, rhs: T) {
                for i in 0..self.data.len() {
                    self[i] = self[i] - rhs;
                }
            }
        }
    };
}

#[macro_export]
macro_rules! setup_mul {
    ($t:ty, $a:tt) => {
        impl<T> Mul for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T>,
        {
            type Output = $t;

            fn mul(mut self, rhs: Self) -> Self::Output {
                for i in 0..self.data.len() {
                    self[i] = self[i] * rhs[i];
                }
                return self;
            }
        }
        /*
        impl<T> Mul for &$t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
            type Output = $t;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..self.data.len() { output[i] = self[i] * rhs[i]; }
                return output;
            }
        }
        */
        impl<T> MulAssign for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T>,
        {
            fn mul_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() {
                    self[i] = self[i] * rhs[i];
                }
            }
        }
        impl<T> Mul<T> for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T>,
        {
            type Output = $t;

            fn mul(mut self, rhs: T) -> Self::Output {
                for i in 0..self.data.len() {
                    self[i] = self[i] * rhs;
                }
                return self;
            }
        }
        impl<T> MulAssign<T> for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T>,
        {
            fn mul_assign(&mut self, rhs: T) {
                for i in 0..self.data.len() {
                    self[i] = self[i] * rhs;
                }
            }
        }
    };
}

#[macro_export]
macro_rules! setup_div {
    ($t:ty, $a:tt) => {
        impl<T> Div for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T>,
        {
            type Output = $t;

            fn div(mut self, rhs: Self) -> Self::Output {
                for i in 0..self.data.len() {
                    self[i] = self[i] / rhs[i];
                }
                return self;
            }
        }
        /*
        impl<T> Div for &$t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
            type Output = $t;

            fn div(self, rhs: Self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..self.data.len() { output[i] = self[i] / rhs[i]; }
                return output;
            }
        }
        */
        impl<T> DivAssign for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T>,
        {
            fn div_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() {
                    self[i] = self[i] / rhs[i];
                }
            }
        }
        impl<T> Div<T> for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T>,
        {
            type Output = $t;

            fn div(mut self, rhs: T) -> Self::Output {
                for i in 0..self.data.len() {
                    self[i] = self[i] / rhs;
                }
                return self;
            }
        }
        impl<T> DivAssign<T> for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T>,
        {
            fn div_assign(&mut self, rhs: T) {
                for i in 0..self.data.len() {
                    self[i] = self[i] / rhs;
                }
            }
        }
    };
}

#[macro_export]
macro_rules! setup_una {
    ($t:ty, $a:tt) => {
        impl<T> Neg for $t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Neg<Output = T>,
        {
            type Output = $t;

            fn neg(mut self) -> Self::Output {
                for i in 0..self.data.len() {
                    self[i] = -self[i];
                }
                return self;
            }
        }
        impl<T> Neg for &$t
        where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Neg<Output = T>,
        {
            type Output = $t;

            fn neg(self) -> Self::Output {
                let mut output = <$t>::default_zero();
                for i in 0..self.data.len() {
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
                for i in 0..self.data.len() {
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
                for i in 0..self.data.len() {
                    self[i] /= len;
                }
            }
            // Get the normalized value of the current vector without updating it
            pub fn normalized(self) -> Self {
                let len = self.length();
                let mut output: Self = Self::default_zero();
                for i in 0..self.data.len() {
                    output[i] = self[i] / len;
                }
                return output;
            }
            // Get the dot product between two vectors
            pub fn dot(self, other: Self) -> $f {
                let mut dot: $f = 0.0;
                for i in 0..self.data.len() {
                    dot += self[i] * other[i];
                }
                return dot;
            }
            // Get the min value between two vec3s
            pub fn min(self, other: Self) -> Self {
                let mut min = <$t>::default_zero();
                for i in 0..self.data.len() {
                    min[i] = self[i].min(other[i]);
                }
                return min;
            }
            // Get the max value between two vec3s
            pub fn max(self, other: Self) -> Self {
                let mut min = <$t>::default_zero();
                for i in 0..self.data.len() {
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
        impl<T> $t where
            T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd,
        {
            // Equals
            pub fn elem_eq(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::default_zero();
                for i in 0..self.data.len() {
                    out[i] = self[i] == other[i];
                }
                out
            }
            // Greater then
            pub fn elem_gt(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::default_zero();
                for i in 0..self.data.len() {
                    out[i] = self[i] > other[i];
                }
                out
            }
            // Less than
            pub fn elem_lt(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::default_zero();
                for i in 0..self.data.len() {
                    out[i] = self[i] < other[i];
                }
                out
            }
            // Greater than or equals
            pub fn elem_gte(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::default_zero();
                for i in 0..self.data.len() {
                    out[i] = self[i] >= other[i];
                }
                out
            }
            // Less than or equals
            pub fn elem_lte(&self, other: &Self) -> $out {
                let mut out: $out = <$out>::default_zero();
                for i in 0..self.data.len() {
                    out[i] = self[i] <= other[i];
                }
                out
            }            
        }
        impl $out {
            // Return true if all the elements are true
            pub fn all(&self) -> bool {
                let mut out: bool = true;
                for i in 0..self.data.len() {
                    out &= self[i];
                }
                out
            }
            // Return true if one or more elements are true
            pub fn any(&self) -> bool {
                let mut out: bool = false;
                for i in 0..self.data.len() {
                    out |= self[i];
                }
                out
            }
        }
        impl<T> BitAnd for $t where
            T: DefaultStates + Clone + Copy + Sized + BitAnd + BitAndAssign {
            type Output = $t;
            // Element wise and
            fn bitand(self, rhs: Self) -> Self::Output {
                let mut out: $t = self;
                for i in 0..self.data.len() {
                    out[i] &= rhs[i];
                }
                return out;
            }
        }
        impl<T> BitOr for $t where
            T: DefaultStates + Clone + Copy + Sized + BitOr + BitOrAssign {
            type Output = $t;
            // Element wise or
            fn bitor(self, rhs: Self) -> Self::Output {
                let mut out: $t = self;
                for i in 0..self.data.len() {
                    out[i] |= rhs[i];
                }
                return out;
            }
        }
        impl<T> BitXor for $t where
            T: DefaultStates + Clone + Copy + Sized + BitXor + BitXorAssign {
            type Output = $t;
            // Element wise xor
            fn bitxor(self, rhs: Self) -> Self::Output {
                let mut out: $t = self;
                for i in 0..self.data.len() {
                    out[i] ^= rhs[i];
                }
                return out;
            }
        }
        impl<T> Not for $t where
            T: DefaultStates + Clone + Copy + Sized + Not<Output = T> {
            type Output = $t;
            // Element wise not
            fn not(self) -> Self::Output {
                let mut out: $t = self;
                for i in 0..self.data.len() {
                    out[i] = !self[i];
                }
                return out;
            }
        }
    };
} 
#[macro_export]
macro_rules! impl_eq_hash {
    ($t:ty) => {
        impl Eq for $t {}
        impl Hash for $t {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.data.hash(state);
            }
        }
    };
}
