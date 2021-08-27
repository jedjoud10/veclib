// Math library
#![feature(const_trait_impl)]
mod matrix;
mod vector;
mod vectors;
mod types;
mod tests;
use veclib_derive::setup_addition;
use veclib_derive::setup_subtraction;
use veclib_derive::setup_multiplication;
use veclib_derive::setup_division;