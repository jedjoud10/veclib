#[macro_export]
macro_rules! setup_add {
    ($t:ty, $a:tt) => {
        impl<T> Add for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
            type Output = $t;

            fn add(mut self, rhs: Self) -> Self::Output {
                for i in 0..self.data.len() { self[i] = self[i] + rhs[i]; }
                return self;
            }
        }
        impl<T> Add for &$t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
            type Output = $t;

            fn add(self, rhs: Self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..self.data.len() { output[i] = self[i] + rhs[i]; }
                return output;
            }
        }
        impl<T> AddAssign for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
            fn add_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() { self[i] = self[i] + rhs[i]; }       
            }
        }
        impl<T> Add<T> for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
            type Output = $t;

            fn add(mut self, rhs: T) -> Self::Output {
                for i in 0..self.data.len() { self[i] = self[i] + rhs; }
                return self;
            }
        }
        impl<T> AddAssign<T> for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
            fn add_assign(&mut self, rhs: T) {
                for i in 0..self.data.len() { self[i] = self[i] + rhs; }       
            }
        }
    }
}

#[macro_export]
macro_rules! setup_sub {
    ($t:ty, $a:tt) => {
        impl<T> Sub for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
            type Output = $t;

            fn sub(mut self, rhs: Self) -> Self::Output {
                for i in 0..self.data.len() { self[i] = self[i] - rhs[i]; }
                return self;
            }
        }
        impl<T> Sub for &$t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
            type Output = $t;

            fn sub(self, rhs: Self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..self.data.len() { output[i] = self[i] - rhs[i]; }
                return output;
            }
        }
        impl<T> SubAssign for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
            fn sub_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() { self[i] = self[i] - rhs[i]; }       
            }
        }
        impl<T> Sub<T> for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
            type Output = $t;

            fn sub(mut self, rhs: T) -> Self::Output {
                for i in 0..self.data.len() { self[i] = self[i] - rhs; }
                return self;
            }
        }
        impl<T> SubAssign<T> for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
            fn sub_assign(&mut self, rhs: T) {
                for i in 0..self.data.len() { self[i] = self[i] - rhs; }       
            }
        }
    }
}

#[macro_export]
macro_rules! setup_mul {
    ($t:ty, $a:tt) => {
        impl<T> Mul for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
            type Output = $t;

            fn mul(mut self, rhs: Self) -> Self::Output {
                for i in 0..self.data.len() { self[i] = self[i] * rhs[i]; }
                return self;
            }
        }
        impl<T> Mul for &$t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
            type Output = $t;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..self.data.len() { output[i] = self[i] * rhs[i]; }
                return output;
            }
        }
        impl<T> MulAssign for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
            fn mul_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() { self[i] = self[i] * rhs[i]; }       
            }
        }
        impl<T> Mul<T> for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
            type Output = $t;

            fn mul(mut self, rhs: T) -> Self::Output {
                for i in 0..self.data.len() { self[i] = self[i] * rhs; }
                return self;
            }
        }
        impl<T> MulAssign<T> for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
            fn mul_assign(&mut self, rhs: T) {
                for i in 0..self.data.len() { self[i] = self[i] * rhs; }       
            }
        }
    }
}

#[macro_export]
macro_rules! setup_div {
    ($t:ty, $a:tt) => {
        impl<T> Div for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
            type Output = $t;

            fn div(mut self, rhs: Self) -> Self::Output {
                for i in 0..self.data.len() { self[i] = self[i] / rhs[i]; }
                return self;
            }
        }
        impl<T> Div for &$t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
            type Output = $t;

            fn div(self, rhs: Self) -> Self::Output {
                let mut output = <$t>::ZERO;
                for i in 0..self.data.len() { output[i] = self[i] / rhs[i]; }
                return output;
            }
        }
        impl<T> DivAssign for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
            fn div_assign(&mut self, rhs: Self) {
                for i in 0..self.data.len() { self[i] = self[i] / rhs[i]; }       
            }
        }
        impl<T> Div<T> for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
            type Output = $t;

            fn div(mut self, rhs: T) -> Self::Output {
                for i in 0..self.data.len() { self[i] = self[i] / rhs; }
                return self;
            }
        }
        impl<T> DivAssign<T> for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
            fn div_assign(&mut self, rhs: T) {
                for i in 0..self.data.len() { self[i] = self[i] / rhs; }       
            }
        }
    }
}

#[macro_export]
macro_rules! setup_una {
    ($t:ty, $a:tt) => {
        impl<T> Neg for $t where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Neg<Output = T> {
            type Output = $t;

            fn neg(mut self) -> Self::Output {
                for i in 0..self.data.len() { self[i] = -self[i]; }
                return self;
            }
        }
    }
}