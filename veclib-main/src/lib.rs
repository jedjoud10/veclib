// Math library
#![feature(const_trait_impl)]
mod vector;
mod vectors;
mod types;
mod tests;
mod matrix;
mod quaternion;
use veclib_proc_macros::setup_addition;
use veclib_proc_macros::setup_subtraction;
use veclib_proc_macros::setup_multiplication;
use veclib_proc_macros::setup_division;
// Export the types
pub use vectors::Vector2;
pub use vectors::Vector3;
pub use vectors::Vector4;
pub use matrix::Matrix4x4;
pub use quaternion::Quaternion;