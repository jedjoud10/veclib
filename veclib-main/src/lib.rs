// Math library
#![feature(const_trait_impl)]
mod matrix;
mod vector;
mod vectors;
mod types;
mod tests;

extern crate proc_macro;
use proc_macro::TokenStream;

// Macros
#[proc_macro]
pub fn addition_operator(_item: TokenStream) -> TokenStream {
    "
    impl<T> Add for Vector2<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
        type Output = Vector2<T>;
        
        fn add(mut self, rhs: Self) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] + rhs[i]; }
            return self;
        }
    }
    impl<T> AddAssign for Vector2<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
        fn add_assign(&mut self, rhs: Self) {
            for i in 0..self.data.len() { self[i] = self[i] + rhs[i]; }       
        }
    }
    impl<T> Add<T> for Vector2<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
        type Output = Vector2<T>;
        
        fn add(mut self, rhs: T) -> Self::Output {
            for i in 0..self.data.len() { self[i] = self[i] + rhs; }
            return self;
        }
    }
    impl<T> AddAssign<T> for Vector2<T> where T: DefaultStates + Clone + Copy + Sized + PartialEq + PartialOrd + Add<Output = T> {
        fn add_assign(&mut self, rhs: T) {
            for i in 0..self.data.len() { self[i] = self[i] + rhs; }       
        }
    }
    ".parse().unwrap()
}

#[macro_use]
extern crate addition_operator;