use crate::vectors::*;

// Const functions
pub const fn vec2<T>(f1: T, f2: T) -> Vector2<T> {
    Vector2 { x: f1, y: f2 }
}
pub const fn vec3<T>(f1: T, f2: T, f3: T) -> Vector3<T> {
    Vector3 { x: f1, y: f2, z: f3 }
}
pub const fn vec4<T>(f1: T, f2: T, f3: T, f4: T) -> Vector4<T> {
    Vector4 { x: f1, y: f2, z: f3, w: f4 }
}
