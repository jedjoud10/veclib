// Math library
mod matrix;
mod quaternion;
mod cast;
//mod tests;
mod vectors;
// Export the types
pub use matrix::*;
use num::{Zero, One};
pub use quaternion::*;
pub use vectors::*;


fn test() {
    let v = crate::Vector2::<f32>::zero();
    let v2 = crate::Vector2::<f32>::zero();
    let distance = crate::Vector2::distance(v, v2);
    
    /*
    let v2 = crate::vectors::cast::<f32, f64>(v);
    */
}