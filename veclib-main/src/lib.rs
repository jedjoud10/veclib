// Math library
#![feature(const_trait_impl)]
mod matrix;
mod vector;
mod vectors;
mod types;
mod tests;
use veclib_proc_macros::setup_addition;
use veclib_proc_macros::setup_subtraction;
use veclib_proc_macros::setup_multiplication;
use veclib_proc_macros::setup_division;