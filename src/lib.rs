// Math library
#![feature(const_trait_impl)]
mod macros;
mod matrix;
mod quaternion;
mod tests;
mod types;
mod vector;
mod vectors;
// Export the types
pub use matrix::Matrix4x4;
pub use quaternion::Quaternion;
pub use vector::Swizzable;
pub use vectors::Vector2;
pub use vectors::Vector3;
pub use vectors::Vector4;
