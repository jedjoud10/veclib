use std::{iter::Copied, ops::{Index, IndexMut, Mul}};

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

impl<T> Matrix4x4<T> where T: DefaultStates + Clone + Copy {
    // Transpose the matrix
    pub fn transpose(&mut self) {
        for x in 0..4 {
            for y in 0..4 {
                self[x][y] = self[y][x];
            }
        }
    }
}

// Creation code for the matrix
#[allow(dead_code)]
impl Matrix4x4<f32> {
    // Create a matrix from 4 vector4s
    pub fn new(vec1: Vector4<f32>, vec2: Vector4<f32>, vec3: Vector4<f32>, vec4: Vector4<f32>) -> Self {
        return Matrix4x4 { data: [vec1, vec2, vec3, vec4] };
    }
    // Create a perspective projection matrix
    // Bonk https://gamedev.stackexchange.com/questions/120338/what-does-a-perspective-projection-matrix-look-like-in-opengl
    pub fn from_perspective(near: f32, far: f32, aspect: f32, fov: f32) -> Self {
        // Math
        let first = 1.0_f32 / (aspect * (fov / 2.0).tan());
        let second = 1.0_f32 / (fov / 2.0).tan();
        // The output
        let mut matrix: Self = Self::default_identity();
        // Right now it is using row major but I will switch it to collumn major later
        // This is row major
        matrix[0] = Vector4::new(first, 0.0, 0.0, 0.0);
        matrix[1] = Vector4::new(0.0, second, 0.0, 0.0);
        matrix[2] = Vector4::new(0.0, 0.0, -((far + near) / (far - near)), -((2.0 * far * near) / (far - near)));
        matrix[3] = Vector4::default_z();
        matrix.transpose();
        // Transpose the matrix
        matrix
    }
    // Create a translation matrix
    pub fn from_translation(position: Vector3<f32>) -> Self {
        // The output
        let mut matrix: Self = Self::default_identity();
        matrix[0] = Vector4::default_x();
        matrix[1] = Vector4::default_y();
        matrix[2] = Vector4::default_z();
        matrix[3] = Vector4::new(position[0], position[1], position[2], 1.0);
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

        let mut output: Matrix4x4<f32> = Matrix4x4::<f32> {
            data: [
                Vector4::<f32>::new(xaxis.x(), xaxis.y(), xaxis.z(), -xaxis.dot(*eye)),
                Vector4::<f32>::new(yaxis.x(), yaxis.y(), yaxis.z(), -yaxis.dot(*eye)),
                Vector4::<f32>::new(zaxis.x(), zaxis.y(), zaxis.z(), -zaxis.dot(*eye)),
                Vector4::<f32>::default_w(),
            ],
        };

        // Transpose the matrix
        output.transpose();

        return output;
    }
    // Create a rotation matrix
    pub fn from_quaternion(quat: &Quaternion<f32>) -> Self {
        let qx = quat[0];
        let qy = quat[1];
        let qz = quat[2];
        let qw = quat[3];
        let vec1 = Vector4::<f32>::new(1.0 - 2.0 * qy * qy - 2.0 * qz * qz, 2.0 * qx * qy - 2.0 * qz * qw, 2.0 * qx * qz + 2.0 * qy * qw, 0.0);
        let vec2 = Vector4::<f32>::new(2.0 * qx * qy + 2.0 * qz * qw, 1.0 - 2.0 * qx * qx - 2.0 * qz * qz, 2.0 * qy * qz - 2.0 * qx * qw, 0.0);
        let vec3 = Vector4::<f32>::new(2.0 * qx * qz - 2.0 * qy * qw, 2.0 * qy * qz + 2.0 * qx * qw, 1.0 - 2.0 * qx * qx - 2.0 * qy * qy, 0.0);
        let vec4 = Vector4::<f32>::default_w();
        return Matrix4x4::new(vec1, vec2, vec3, vec4);
    }
    // Create a scale matrix
    pub fn from_scale(scale: Vector3<f32>) -> Self {
        // Too good bro
        return Matrix4x4::new(
            Vector4::default_x() * scale.x(),
            Vector4::default_y() * scale.y(),
            Vector4::default_z() * scale.z(),
            Vector4::default_w(),
        );
    }
    // Multiply a matrix by this matrix
    pub fn mul_mat4x4(&self, other: Matrix4x4<f32>) -> Self {
        let mut output: Self = Self::default_identity();
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
                let b = other[x];
                output[x][y] = a.dot(b);
            }
        }
        return output;
    }
    // Return the inverse of this matrix
    pub fn inverse(&self) -> Self {
        todo!();
    }
}
// Multiply this matrix by another matrix
impl Mul for Matrix4x4<f32> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        return self.mul_mat4x4(rhs);
    }
}
// Transform a vector by the matrix
impl Matrix4x4<f32> {
    // Transform a 4D vector by the matrix
    pub fn mul_vector(&self, vector: &Vector4<f32>) -> Vector4<f32> {
        // Multiply the vector by this matrix
        let x = self[0].dot(*vector);
        let y = self[1].dot(*vector);
        let z = self[2].dot(*vector);
        let w = self[3].dot(*vector);
        return Vector4::<f32>::new(x, y, z, w);
    }
    // Transform a 3D point by the matrix, basically create a 4D vector out of it with the W component being 1.0
    pub fn mul_point(&self, point: &Vector3<f32>) -> Vector3<f32> {
        self.mul_vector(&Vector4::new(point.x(), point.y(), point.z(), 1.0)).get3([0, 1, 2])
    }
}
