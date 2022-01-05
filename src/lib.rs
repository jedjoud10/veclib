// Math library
pub mod consts;
mod macros;
mod matrix;
mod quaternion;
mod tests;
mod types;
mod vector;
mod vectors;
// Export the types
pub use matrix::Matrix4x4;
pub use quaternion::EulerAnglesOrder;
pub use quaternion::Quaternion;
pub use types::DefaultStates;
pub use vector::Swizzable;
pub use vectors::*;
