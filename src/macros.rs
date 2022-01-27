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
        impl SupportedValue for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
        }
    };
}

#[macro_export]
macro_rules! setup_add {
    ($t:ty, $a:tt) => {
        impl<T> Add for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Add<Output = T>,
        {
            type Output = $t;

            fn add(mut self, rhs: Self) -> Self::Output {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] + rhs[i];
                }
                return self;
            }
        }
        impl<T> Add for &$t
        where
            T: SupportedValue + PartialEq + PartialOrd + Add<Output = T>,
        {
            type Output = $t;

            fn add(self, rhs: Self) -> Self::Output {
                let mut me = *self;
                let rhs = *rhs;
                for i in 0..Self::ELEM_COUNT {
                    me[i] = me[i] + rhs[i];
                }
                return me;
            }
        }
        impl<T> Add<T> for &$t
        where
            T: SupportedValue + PartialEq + PartialOrd + Add<Output = T>,
        {
            type Output = $t;

            fn add(self, rhs: T) -> Self::Output {
                let mut me = *self;
                let rhs = rhs;
                for i in 0..Self::ELEM_COUNT {
                    me[i] = me[i] + rhs;
                }
                return me;
            }
        }
        impl<T> AddAssign for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Add<Output = T>,
        {
            fn add_assign(&mut self, rhs: Self) {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] + rhs[i];
                }
            }
        }
        impl<T> Add<T> for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Add<Output = T>,
        {
            type Output = $t;

            fn add(mut self, rhs: T) -> Self::Output {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] + rhs;
                }
                return self;
            }
        }
        impl<T> AddAssign<T> for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Add<Output = T>,
        {
            fn add_assign(&mut self, rhs: T) {
                for i in 0..Self::ELEM_COUNT {
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
            T: SupportedValue + PartialEq + PartialOrd + Sub<Output = T>,
        {
            type Output = $t;

            fn sub(mut self, rhs: Self) -> Self::Output {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] - rhs[i];
                }
                return self;
            }
        }
        impl<T> Sub for &$t
        where
            T: SupportedValue + PartialEq + PartialOrd + Sub<Output = T>,
        {
            type Output = $t;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut me = *self;
                let rhs = *rhs;
                for i in 0..Self::ELEM_COUNT {
                    me[i] = me[i] - rhs[i];
                }
                return me;
            }
        }
        impl<T> Sub<T> for &$t
        where
            T: SupportedValue + PartialEq + PartialOrd + Sub<Output = T>,
        {
            type Output = $t;

            fn sub(self, rhs: T) -> Self::Output {
                let mut me = *self;
                let rhs = rhs;
                for i in 0..Self::ELEM_COUNT {
                    me[i] = me[i] - rhs;
                }
                return me;
            }
        }
        impl<T> SubAssign for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Sub<Output = T>,
        {
            fn sub_assign(&mut self, rhs: Self) {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] - rhs[i];
                }
            }
        }
        impl<T> Sub<T> for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Sub<Output = T>,
        {
            type Output = $t;

            fn sub(mut self, rhs: T) -> Self::Output {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] - rhs;
                }
                return self;
            }
        }
        impl<T> SubAssign<T> for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Sub<Output = T>,
        {
            fn sub_assign(&mut self, rhs: T) {
                for i in 0..Self::ELEM_COUNT {
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
            T: SupportedValue + PartialEq + PartialOrd + Mul<Output = T>,
        {
            type Output = $t;

            fn mul(mut self, rhs: Self) -> Self::Output {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] * rhs[i];
                }
                return self;
            }
        }
        impl<T> Mul for &$t
        where
            T: SupportedValue + PartialEq + PartialOrd + Mul<Output = T>,
        {
            type Output = $t;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut me = *self;
                let rhs = *rhs;
                for i in 0..Self::ELEM_COUNT {
                    me[i] = me[i] * rhs[i];
                }
                return me;
            }
        }
        impl<T> Mul<T> for &$t
        where
            T: SupportedValue + PartialEq + PartialOrd + Mul<Output = T>,
        {
            type Output = $t;

            fn mul(self, rhs: T) -> Self::Output {
                let mut me = *self;
                let rhs = rhs;
                for i in 0..Self::ELEM_COUNT {
                    me[i] = me[i] * rhs;
                }
                return me;
            }
        }
        impl<T> MulAssign for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Mul<Output = T>,
        {
            fn mul_assign(&mut self, rhs: Self) {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] * rhs[i];
                }
            }
        }
        impl<T> Mul<T> for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Mul<Output = T>,
        {
            type Output = $t;

            fn mul(mut self, rhs: T) -> Self::Output {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] * rhs;
                }
                return self;
            }
        }
        impl<T> MulAssign<T> for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Mul<Output = T>,
        {
            fn mul_assign(&mut self, rhs: T) {
                for i in 0..Self::ELEM_COUNT {
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
            T: SupportedValue + PartialEq + PartialOrd + Div<Output = T>,
        {
            type Output = $t;

            fn div(mut self, rhs: Self) -> Self::Output {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] / rhs[i];
                }
                return self;
            }
        }
        impl<T> Div for &$t
        where
            T: SupportedValue + PartialEq + PartialOrd + Div<Output = T>,
        {
            type Output = $t;

            fn div(self, rhs: Self) -> Self::Output {
                let mut me = *self;
                let rhs = *rhs;
                for i in 0..Self::ELEM_COUNT {
                    me[i] = me[i] / rhs[i];
                }
                return me;
            }
        }
        impl<T> Div<T> for &$t
        where
            T: SupportedValue + PartialEq + PartialOrd + Div<Output = T>,
        {
            type Output = $t;

            fn div(self, rhs: T) -> Self::Output {
                let mut me = *self;
                let rhs = rhs;
                for i in 0..Self::ELEM_COUNT {
                    me[i] = me[i] / rhs;
                }
                return me;
            }
        }
        impl<T> DivAssign for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Div<Output = T>,
        {
            fn div_assign(&mut self, rhs: Self) {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] / rhs[i];
                }
            }
        }
        impl<T> Div<T> for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Div<Output = T>,
        {
            type Output = $t;

            fn div(mut self, rhs: T) -> Self::Output {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] / rhs;
                }
                return self;
            }
        }
        impl<T> DivAssign<T> for $t
        where
            T: SupportedValue + PartialEq + PartialOrd + Div<Output = T>,
        {
            fn div_assign(&mut self, rhs: T) {
                for i in 0..Self::ELEM_COUNT {
                    self[i] = self[i] / rhs;
                }
            }
        }
    };
}

#[macro_export]
macro_rules! setup_neg {
    ($t:ty, $a:tt) => {
        impl<T> Neg for $t
        where
            T: SupportedValue + Neg<Output = T> + Copy,
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
            T: SupportedValue + Neg<Output = T> + Copy,
        {
            type Output = $t;

            fn neg(self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..(<$t>::ELEM_COUNT) {
                    output[i] = -self[i];
                }
                return output;
            }
        }
    };
}

#[macro_export]
macro_rules! setup_vector_operations {
    ($t:ty, $a:tt, $f: ty) => {
        // Setup the shared vector operations
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
            T: SupportedValue + PartialEq + PartialOrd,
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
            pub fn select<T: Clone + Copy>(&self, first: &$t, second: &$t) -> $t {
                let mut output_vector: $t = (*first).clone();
                for i in 0..Self::ELEM_COUNT {
                    output_vector[i] = if self[i] { first[i] } else { second[i] }
                }
                return output_vector;
            }
        }
        impl<T> BitAnd for $t
        where
            T: SupportedValue + BitAnd + BitAndAssign,
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
            T: SupportedValue + BitOr + BitOrAssign,
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
            T: SupportedValue + BitXor + BitXorAssign,
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
            T: SupportedValue + Not<Output = T>,
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
macro_rules! impl_eq_hash {
    ($t:ty) => {
        impl Eq for $t {}
        impl Hash for $t {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                for i in 0..<$t>::ELEM_COUNT {
                    self[i].hash(state);
                }
            }
        }
    };
}
#[macro_export]
macro_rules! impl_matrix {
    ($t: ty, $f: ty) => {
        // Creation code for the matrix
        #[allow(dead_code)]
        impl $t {
            // Create a matrix from 4 vector4s
            pub fn new(vec1: Vector4<$f>, vec2: Vector4<$f>, vec3: Vector4<$f>, vec4: Vector4<$f>) -> Self {
                Matrix4x4 { data: [vec1, vec2, vec3, vec4] }
            }
            // Create a perspective projection matrix
            // Bonk https://gamedev.stackexchange.com/questions/120338/what-does-a-perspective-projection-matrix-look-like-in-opengl
            pub fn from_perspective(near: $f, far: $f, aspect: $f, fov: $f) -> Self {
                // Math
                let first = 1.0 / (aspect * (fov / 2.0).tan());
                let second = 1.0 / (fov / 2.0).tan();
                // The output
                let mut matrix: Self = Self::IDENTITY;
                // Right now it is using row major but I will switch it to collumn major later
                // This is row major
                *matrix.get_vec_mut(0) = Vector4::new(first, 0.0, 0.0, 0.0);
                *matrix.get_vec_mut(1) = Vector4::new(0.0, second, 0.0, 0.0);
                *matrix.get_vec_mut(2) = Vector4::new(0.0, 0.0, -((far + near) / (far - near)), -((2.0 * far * near) / (far - near)));
                *matrix.get_vec_mut(3) = -Vector4::Z;
                matrix.transpose();
                // Transpose the matrix
                matrix
            }
            // Create a translation matrix
            pub fn from_translation(position: Vector3<$f>) -> Self {
                // The output
                let mut matrix: Self = Self::IDENTITY;
                *matrix.get_vec_mut(0) = Vector4::X;
                *matrix.get_vec_mut(1) = Vector4::Y;
                *matrix.get_vec_mut(2) = Vector4::Z;
                *matrix.get_vec_mut(3) = Vector4::new(position[0], position[1], position[2], 1.0);
                matrix
            }
            // Create a look at matrix
            // https://www.geertarien.com/blog/2017/07/30/breakdown-of-the-lookAt-function-in-OpenGL/
            pub fn look_at(eye: &Vector3<$f>, up: &Vector3<$f>, target: &Vector3<$f>) -> Self {
                // The output
                let zaxis: Vector3<$f> = (*target - *eye).normalized();
                let xaxis: Vector3<$f> = zaxis.cross(*up);
                let yaxis: Vector3<$f> = xaxis.cross(zaxis);

                let zaxis = -zaxis;

                let mut output: Matrix4x4<$f> = Matrix4x4::<$f> {
                    data: [
                        Vector4::<$f>::new(xaxis.x, xaxis.y, xaxis.z, -xaxis.dot(*eye)),
                        Vector4::<$f>::new(yaxis.x, yaxis.y, yaxis.z, -yaxis.dot(*eye)),
                        Vector4::<$f>::new(zaxis.x, zaxis.y, zaxis.z, -zaxis.dot(*eye)),
                        Vector4::<$f>::W,
                    ],
                };

                // Transpose the matrix
                output.transpose();

                output
            }
            // Create a rotation matrix
            pub fn from_quaternion(quat: &Quaternion<$f>) -> Self {
                let qx = quat[0];
                let qy = quat[1];
                let qz = quat[2];
                let qw = quat[3];
                let vec1 = Vector4::<$f>::new(1.0 - 2.0 * qy * qy - 2.0 * qz * qz, 2.0 * qx * qy - 2.0 * qz * qw, 2.0 * qx * qz + 2.0 * qy * qw, 0.0);
                let vec2 = Vector4::<$f>::new(2.0 * qx * qy + 2.0 * qz * qw, 1.0 - 2.0 * qx * qx - 2.0 * qz * qz, 2.0 * qy * qz - 2.0 * qx * qw, 0.0);
                let vec3 = Vector4::<$f>::new(2.0 * qx * qz - 2.0 * qy * qw, 2.0 * qy * qz + 2.0 * qx * qw, 1.0 - 2.0 * qx * qx - 2.0 * qy * qy, 0.0);
                let vec4 = Vector4::<$f>::W;
                Matrix4x4::<$f>::new(vec1, vec2, vec3, vec4)
            }
            // Create a scale matrix
            pub fn from_scale(scale: Vector3<$f>) -> Self {
                // Too good bro
                Matrix4x4::<$f>::new(Vector4::X * scale.x, Vector4::Y * scale.y, Vector4::Z * scale.z, Vector4::W)
            }
            // Multiply a matrix by this matrix
            pub fn mul_mat4x4(&self, other: Matrix4x4<$f>) -> Self {
                let mut output: Self = Self::IDENTITY;
                // Get the A vectors
                let mut a_vectors: [Vector4<$f>; 4] = [Vector4::<$f>::ZERO; 4];
                for y in 0..4 {
                    a_vectors[y][0] = self.get_vec(0)[y];
                    a_vectors[y][1] = self.get_vec(1)[y];
                    a_vectors[y][2] = self.get_vec(2)[y];
                    a_vectors[y][3] = self.get_vec(3)[y];
                }
                // Get the dot product
                for y in 0..4 {
                    for x in 0..4 {
                        // Collumn major
                        // Get A
                        let a: Vector4<$f> = a_vectors[y];
                        let b = *other.get_vec(x);
                        let h = &mut output.get_vec_mut(x)[y];
                        *h = a.dot(b);
                    }
                }
                output
            }
            // Return the inverse of this matrix
            // https://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix/44446912#44446912
            pub fn inverse(&self, inverse: &mut Self) -> bool {
                let m = *self;
                let mut inv = Self::default();

                inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15] + m[9] * m[7] * m[14] + m[13] * m[6] * m[11] - m[13] * m[7] * m[10];

                inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15] - m[8] * m[7] * m[14] - m[12] * m[6] * m[11] + m[12] * m[7] * m[10];

                inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15] + m[8] * m[7] * m[13] + m[12] * m[5] * m[11] - m[12] * m[7] * m[9];

                inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14] - m[8] * m[6] * m[13] - m[12] * m[5] * m[10] + m[12] * m[6] * m[9];

                inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15] - m[9] * m[3] * m[14] - m[13] * m[2] * m[11] + m[13] * m[3] * m[10];

                inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15] + m[8] * m[3] * m[14] + m[12] * m[2] * m[11] - m[12] * m[3] * m[10];

                inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15] - m[8] * m[3] * m[13] - m[12] * m[1] * m[11] + m[12] * m[3] * m[9];

                inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14] + m[8] * m[2] * m[13] + m[12] * m[1] * m[10] - m[12] * m[2] * m[9];

                inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15] + m[5] * m[3] * m[14] + m[13] * m[2] * m[7] - m[13] * m[3] * m[6];

                inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15] - m[4] * m[3] * m[14] - m[12] * m[2] * m[7] + m[12] * m[3] * m[6];

                inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15] + m[4] * m[3] * m[13] + m[12] * m[1] * m[7] - m[12] * m[3] * m[5];

                inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14] - m[4] * m[2] * m[13] - m[12] * m[1] * m[6] + m[12] * m[2] * m[5];

                inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11] - m[5] * m[3] * m[10] - m[9] * m[2] * m[7] + m[9] * m[3] * m[6];

                inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11] + m[4] * m[3] * m[10] + m[8] * m[2] * m[7] - m[8] * m[3] * m[6];

                inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11] - m[4] * m[3] * m[9] - m[8] * m[1] * m[7] + m[8] * m[3] * m[5];

                inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10] + m[4] * m[2] * m[9] + m[8] * m[1] * m[6] - m[8] * m[2] * m[5];
                let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
                // Not valid
                if det == 0.0 {
                    return false;
                }

                for i in 0..16 {
                    inverse[i] = inv[i] * (1.0 / det);
                }
                true
            }
            // Inversed
            pub fn inversed(&self) -> Self {
                let mut output = Self::IDENTITY;
                self.inverse(&mut output);
                output
            }
        }
    };
}

#[macro_export]
macro_rules! impl_quaternion {
    ($t: ty, $f: ty) => {
        // Da code
        impl Quaternion<$f> {
            // Create a quaternion from euler angles and the order of the angles operation
            // https://www.euclideanspace.com/maths/geometry/rotations/conversions/angleToQuaternion/index.htm
            pub fn from_euler_angles(order: EulerAnglesOrder, euler: Vector3<$f>) -> Quaternion<$f> {
                let output: Quaternion<$f>;
                match order {
                    EulerAnglesOrder::XYZ => {
                        output = Self::from_z_angle(euler.z) * Self::from_y_angle(euler.y) * Self::from_x_angle(euler.x);
                    }
                    EulerAnglesOrder::XZY => {
                        output = Self::from_y_angle(euler.y) * Self::from_z_angle(euler.z) * Self::from_x_angle(euler.x);
                    }
                    EulerAnglesOrder::YXZ => {
                        output = Self::from_z_angle(euler.z) * Self::from_x_angle(euler.x) * Self::from_y_angle(euler.y);
                    }
                    EulerAnglesOrder::YZX => {
                        output = Self::from_x_angle(euler.x) * Self::from_z_angle(euler.z) * Self::from_y_angle(euler.y);
                    }
                    EulerAnglesOrder::ZXY => {
                        output = Self::from_y_angle(euler.y) * Self::from_x_angle(euler.x) * Self::from_z_angle(euler.z);
                    }
                    EulerAnglesOrder::ZYX => {
                        output = Self::from_x_angle(euler.x) * Self::from_y_angle(euler.y) * Self::from_z_angle(euler.z);
                    }
                }
                output
            }
            // Create a quaternion from an angle and an axis
            pub fn from_axis_angle(axis: Vector3<$f>, angle: $f) -> Quaternion<$f> {
                // Normalize just in case
                //axis.normalize();
                let x = (angle / 2.0).sin();
                let mut output: Quaternion<$f> = Self::IDENTITY;
                output[0] = axis.x * x;
                output[1] = axis.y * x;
                output[2] = axis.z * x;
                output[3] = (angle / 2.0).cos();
                /*
                let mut output = Quaternion {
                    data: Vector4::<f32>::new(axis.x, axis.y, axis.z, angle)
                };
                */
                output
            }
            // Create the quaternion from an angle and the X axis
            pub fn from_x_angle(angle: $f) -> Quaternion<$f> {
                Self::from_axis_angle(Vector3::X, angle)
            }
            // Create the quaternion from an angle and the Y axis
            pub fn from_y_angle(angle: $f) -> Quaternion<$f> {
                Self::from_axis_angle(Vector3::Y, angle)
            }
            // Create the quaternion from an angle and the Z axis
            pub fn from_z_angle(angle: $f) -> Quaternion<$f> {
                Self::from_axis_angle(Vector3::Z, angle)
            }

            // Transform a point by this quaternion
            pub fn mul_point(&self, point: Vector3<$f>) -> Vector3<$f> {
                // Turn the vector into a pure quaternion
                let mut pure: Quaternion<$f> = Self::IDENTITY;
                let self_vector = self.data.get3([0, 1, 2]);
                pure[3] = 0.0;
                pure[0] = point[0];
                pure[1] = point[1];
                pure[2] = point[2];
                let vector: Vector3<$f> = self_vector.cross(point);
                point + vector * (2.0 * self[3]) + self_vector.cross(vector) * 2.0
            }
            // Multiply a quaternion by this quaternion
            pub fn mul_quaternion(&self, other: Quaternion<$f>) -> Quaternion<$f> {
                // The output
                let mut output: Self = Self::IDENTITY;
                let other_vector = other.data.get3([0, 1, 2]);
                let self_vector = self.data.get3([0, 1, 2]);
                output[3] = self[3] * other[3] + self_vector.dot(other_vector);
                let new_vector: Vector3<$f> = self_vector * other[3] + other_vector * self[3] + self_vector.cross(other_vector);
                output[0] = new_vector.x;
                output[1] = new_vector.y;
                output[2] = new_vector.z;
                output
            }
            // Normalize this quaternion
            pub fn normalize(&mut self) {
                self.data.normalize();
            }
        }

        // Operators
        impl Mul for Quaternion<$f> {
            type Output = Quaternion<$f>;

            fn mul(self, rhs: Self) -> Self::Output {
                //return self.mul_quaternion(rhs);
                rhs.mul_quaternion(self)
            }
        }
    };
}
