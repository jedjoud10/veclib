extern crate proc_macro;
use proc_macro::TokenStream;

// Macros
#[macro_export]
#[proc_macro]
pub fn setup_addition(item: TokenStream) -> TokenStream {
    // The type of vector (Vector2, Vector3, Vector4)
    let vector_type: u8 = item.to_string().parse::<u8>().unwrap();
    "
    impl<T> Add for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
        type Output = Vector{a_a}<T>;
        
        fn add(mut self, rhs: Self) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] + rhs[i]; }
            return self;
        }
    }
    impl<T> AddAssign for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
        fn add_assign(&mut self, rhs: Self) {
            for i in 0..self.data.len() { self[i] = self[i] + rhs[i]; }       
        }
    }
    impl<T> Add<T> for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
        type Output = Vector{a_a}<T>;
        
        fn add(mut self, rhs: T) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] + rhs; }
            return self;
        }
    }
    impl<T> AddAssign<T> for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
        fn add_assign(&mut self, rhs: T) {
            for i in 0..self.data.len() { self[i] = self[i] + rhs; }       
        }
    }
    ".replace("{a_a}", vector_type.to_string().as_str()).parse().unwrap()
}

#[proc_macro]
pub fn setup_subtraction(item: TokenStream) -> TokenStream {
    // The type of vector (Vector2, Vector3, Vector4)
    let vector_type: u8 = item.to_string().parse::<u8>().unwrap();
    "
    impl<T> Sub for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
        type Output = Vector{a_a}<T>;
        
        fn sub(mut self, rhs: Self) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] - rhs[i]; }
            return self;
        }
    }
    impl<T> SubAssign for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
        fn sub_assign(&mut self, rhs: Self) {
            for i in 0..self.data.len() { self[i] = self[i] - rhs[i]; }       
        }
    }
    impl<T> Sub<T> for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
        type Output = Vector{a_a}<T>;
        
        fn sub(mut self, rhs: T) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] - rhs; }
            return self;
        }
    }
    impl<T> SubAssign<T> for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Sub<Output = T> {
        fn sub_assign(&mut self, rhs: T) {
            for i in 0..self.data.len() { self[i] = self[i] - rhs; }       
        }
    }
    ".replace("{a_a}", vector_type.to_string().as_str()).parse().unwrap()
}

#[proc_macro]
pub fn setup_multiplication(item: TokenStream) -> TokenStream {
    // The type of vector (Vector2, Vector3, Vector4)
    let vector_type: u8 = item.to_string().parse::<u8>().unwrap();
    "
    impl<T> Mul for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
        type Output = Vector{a_a}<T>;
        
        fn mul(mut self, rhs: Self) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] * rhs[i]; }
            return self;
        }
    }
    impl<T> MulAssign for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
        fn mul_assign(&mut self, rhs: Self) {
            for i in 0..self.data.len() { self[i] = self[i] * rhs[i]; }       
        }
    }
    impl<T> Mul<T> for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
        type Output = Vector{a_a}<T>;
        
        fn mul(mut self, rhs: T) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] * rhs; }
            return self;
        }
    }
    impl<T> MulAssign<T> for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Mul<Output = T> {
        fn mul_assign(&mut self, rhs: T) {
            for i in 0..self.data.len() { self[i] = self[i] * rhs; }       
        }
    }
    ".replace("{a_a}", vector_type.to_string().as_str()).parse().unwrap()
}

#[proc_macro]
pub fn setup_division(item: TokenStream) -> TokenStream {
    // The type of vector (Vector2, Vector3, Vector4)
    let vector_type: u8 = item.to_string().parse::<u8>().unwrap();
    "
    impl<T> Div for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
        type Output = Vector{a_a}<T>;
        
        fn div(mut self, rhs: Self) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] / rhs[i]; }
            return self;
        }
    }
    impl<T> DivAssign for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
        fn div_assign(&mut self, rhs: Self) {
            for i in 0..self.data.len() { self[i] = self[i] / rhs[i]; }       
        }
    }
    impl<T> Div<T> for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
        type Output = Vector{a_a}<T>;
        
        fn div(mut self, rhs: T) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] / rhs; }
            return self;
        }
    }
    impl<T> DivAssign<T> for Vector{a_a}<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Div<Output = T> {
        fn div_assign(&mut self, rhs: T) {
            for i in 0..self.data.len() { self[i] = self[i] / rhs; }       
        }
    }
    ".replace("{a_a}", vector_type.to_string().as_str()).parse().unwrap()
}