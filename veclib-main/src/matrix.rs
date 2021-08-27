use std::{iter::Copied, ops::{Add, Div, Index, IndexMut}};

use crate::{Quaternion, types::DefaultStates, vectors::{Vector3, Vector4}};

// A simple f32 matrix made of 4 f32/f64 vectors
// TODO: Turn this into a generic struct
pub struct Matrix4x4<T> where T: DefaultStates + Clone + Copy {
    pub data: [Vector4<T>; 4],
}

// Indexer
impl<T> Index<usize> for Matrix4x4<T> where T: DefaultStates + Clone + Copy {
    type Output = Vector4<T>;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[index];
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Matrix4x4<T> where T: DefaultStates + Clone + Copy {
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return &mut self.data[index];
    }
}

// Creation code for the matrix
#[allow(dead_code)]
impl Matrix4x4<f32> {
    // Constants
    pub const IDENTITY: Self = Matrix4x4 {
        data: [Vector4::X, Vector4::Y, Vector4::Z, Vector4::W]
    };
    // Create a perspective projection matrix
    // Bonk https://www.youtube.com/watch?v=U0_ONQQ5ZNM&ab_channel=BrendanGalea
    pub fn from_perspective(near_plane: f32, far_plane: f32, aspect_ratio: f32, y_fov_radians: f32) -> Self {
        // Math
        let first = 1.0_f32 / (aspect_ratio * (y_fov_radians / 2.0).tan());
        let second = 1.0_f32 / (y_fov_radians / 2.0).tan();
        // The output
        let mut matrix: Self = Self::IDENTITY;
        // Remember, this is collumn major
        // Right now it is using row major but I will switch it to collumn major later
        matrix[0] = Vector4::new(first, 0.0, 0.0, 0.0);
        matrix[1] = Vector4::new(0.0, second, 0.0, 0.0);
        matrix[2] = Vector4::new(0.0, 0.0, (2.0 * far_plane) / (far_plane - near_plane), -(far_plane * near_plane) / (far_plane - near_plane));
        matrix[3] = Vector4::Z;
        return matrix;
    }
    // Create a translation matrix
    pub fn from_translation(position: Vector3<f32>) -> Self {
        // The output
        let mut matrix: Self = Self::IDENTITY;
        matrix[0] = Vector4::new(1.0, 0.0, 0.0, position[0]);
        matrix[1] = Vector4::new(0.0, 1.0, 0.0, position[1]);
        matrix[2] = Vector4::new(0.0, 0.0, 1.0, position[2]);
        matrix[3] = Vector4::W;
        return matrix;
    }
    // Create a look at matrix
    // https://www.geertarien.com/blog/2017/07/30/breakdown-of-the-lookAt-function-in-OpenGL/
    pub fn look_at(eye: &Vector3<f32>, up: &Vector3<f32>, target: &Vector3<f32>) -> Self {
        // The output
        let mut matrix: Self = Self::IDENTITY;
        let mut zaxis: Vector3<f32> = (target.clone() - eye.clone()).normalized();    
        let xaxis: Vector3<f32> = zaxis.cross(up);
        let yaxis: Vector3<f32> = xaxis.cross(&zaxis);
        /*
        zaxis = zaxis;

        mat4 viewMatrix = {
          vec4(xaxis.x, xaxis.y, xaxis.z, -dot(xaxis, eye)),
          vec4(yaxis.x, yaxis.y, yaxis.z, -dot(yaxis, eye)),
          vec4(zaxis.x, zaxis.y, zaxis.z, -dot(zaxis, eye)),
          vec4(0, 0, 0, 1)
        };
        */
        todo!();
    }
    // Create a rotation matrix
    pub fn from_quaternion(quat: &Quaternion<f32>) -> Self {
        todo!();
    }
    // Create a scale matrix
    pub fn from_scale(scale: Vector3<f32>) -> Self {
        todo!();
    }
}
// Transform a vector by the matrix
impl Matrix4x4<f32> {
    // Transform a 4D vector by the matrix
    pub fn transform_vector(&self, vector: &Vector4<f32>) -> Vector4<f32>{
        todo!();
    }
}