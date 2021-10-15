// Math library
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
pub use quaternion::EulerAnglesOrder;
pub use vector::Swizzable;
pub use vector::Vector;
pub use types::DefaultStates;
pub use vectors::Vector2;
pub use vectors::Vector3;
pub use vectors::Vector4;
pub use vectors::Vec2Axis;
pub use vectors::Vec3Axis;
pub use vectors::Vec4Axis;
