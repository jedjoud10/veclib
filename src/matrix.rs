use std::{ops::{Index, IndexMut, Mul}};

use crate::{
    types::DefaultStates,
    vector::Swizzable,
    vectors::{Vector3, Vector4},
    Quaternion,
};

// A simple f32 matrix made of 4 f32/f64 vectors
// TODO: Turn this into a generic struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4<T>
where
    T: DefaultStates + Clone + Copy,
{
    pub data: [Vector4<T>; 4],
}

// Default
impl<T> Default for Matrix4x4<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    fn default() -> Self {
        Self::default_identity()
    }
}

// Indexer
impl<T> Index<usize> for Matrix4x4<T>
where
    T: DefaultStates + Clone + Copy,
{
    type Output = Vector4<T>;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Matrix4x4<T>
where
    T: DefaultStates + Clone + Copy,
{
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// Identity matrix available for everyone
impl<T> Matrix4x4<T>
where
    T: DefaultStates + Clone + Copy,
{
    // Defaults
    pub fn default_identity() -> Self {
        Matrix4x4 {
            data: [Vector4::<T>::default_x(), Vector4::<T>::default_y(), Vector4::<T>::default_z(), Vector4::<T>::default_w()],
        }
    }
}

// Creation code for the matrix
#[allow(dead_code)]
impl Matrix4x4<f32> {
    // Create a matrix from 4 vector4s
    pub fn new(vec1: Vector4<f32>, vec2: Vector4<f32>, vec3: Vector4<f32>, vec4: Vector4<f32>) -> Self {
        return Matrix4x4 {
            data: [vec1, vec2, vec3, vec4]
        }
    }
    // Create a perspective projection matrix
    // Bonk https://www.youtube.com/watch?v=U0_ONQQ5ZNM&ab_channel=BrendanGalea
    pub fn from_perspective(near_plane: f32, far_plane: f32, aspect_ratio: f32, y_fov_radians: f32) -> Self {
        // Math
        let first = 1.0_f32 / (aspect_ratio * (y_fov_radians / 2.0).tan());
        let second = 1.0_f32 / (y_fov_radians / 2.0).tan();
        // The output
        let mut matrix: Self = Self::default_identity();
        // Remember, this is collumn major
        // Right now it is using row major but I will switch it to collumn major later
        matrix[0] = Vector4::new(first, 0.0, 0.0, 0.0);
        matrix[1] = Vector4::new(0.0, second, 0.0, 0.0);
        matrix[2] = Vector4::new(0.0, 0.0, (2.0 * far_plane) / (far_plane - near_plane), -(far_plane * near_plane) / (far_plane - near_plane));
        matrix[3] = Vector4::default_z();
        matrix
    }
    // Create a translation matrix
    pub fn from_translation(position: Vector3<f32>) -> Self {
        // The output
        let mut matrix: Self = Self::default_identity();
        matrix[0] = Vector4::new(1.0, 0.0, 0.0, position[0]);
        matrix[1] = Vector4::new(0.0, 1.0, 0.0, position[1]);
        matrix[2] = Vector4::new(0.0, 0.0, 1.0, position[2]);
        matrix[3] = Vector4::default_w();
        matrix
    }
    // Create a look at matrix
    // https://www.geertarien.com/blog/2017/07/30/breakdown-of-the-lookAt-function-in-OpenGL/
    pub fn look_at(eye: &Vector3<f32>, up: &Vector3<f32>, target: &Vector3<f32>) -> Self {
        // The output
        let zaxis: Vector3<f32> = (*target - *eye).normalized();
        let xaxis: Vector3<f32> = zaxis.cross(*up);
        let yaxis: Vector3<f32> = xaxis.cross(zaxis);
        
        let zaxis = -zaxis;

        return Matrix4x4::<f32> {
            data: [
                Vector4::<f32>::new(xaxis.x(), xaxis.y(), xaxis.z(), xaxis.dot(*eye)),
                Vector4::<f32>::new(yaxis.x(), yaxis.y(), yaxis.z(), yaxis.dot(*eye)),
                Vector4::<f32>::new(zaxis.x(), zaxis.y(), zaxis.z(), zaxis.dot(*eye)),
                Vector4::<f32>::default_x(),
            ]
        }
    }
    // Create a rotation matrix
    pub fn from_quaternion(_quat: &Quaternion<f32>) -> Self {
        todo!();
    }
    // Create a scale matrix
    pub fn from_scale(_scale: Vector3<f32>) -> Self {
        todo!();
    }
}
// Multiply this matrix by another matrix
impl Mul for Matrix4x4<f32> {
    type Output = Matrix4x4<f32>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut output: Self = self;
        // Get the A vectors
        let mut a_vectors: [Vector4<f32>; 4] = [Vector4::<f32>::default_zero(); 4];
        for y in 0..4 {
            a_vectors[y][0] = self[0][y];
            a_vectors[y][1] = self[1][y];
            a_vectors[y][2] = self[2][y];
            a_vectors[y][3] = self[3][y];
        }
        // Get the dot product
        for y in 0..4 {
            for x in 0..4 {
                // Collumn major
                // Y is a
                // X is b
                
                // Get A
                let a: Vector4<f32> = a_vectors[y];
                let b = rhs[x];
                output[x][y] = a.dot(b);
            }
        }
        return output;
    }
}
// Transform a vector by the matrix
impl Matrix4x4<f32> {
    // Transform a 4D vector by the matrix
    pub fn transform_vector(&self, _vector: &Vector4<f32>) -> Vector4<f32> {
        todo!();
    }
    // Transform a 3D point by the matrix, basically create a 4D vector out of it with the W component being 1.0
    pub fn transform_point(&self, point: &Vector3<f32>) -> Vector3<f32> {
        self.transform_vector(&Vector4::new(point.x(), point.y(), point.z(), 1.0)).get3([0, 1, 2])
    }
}
