use std::ops::{Index, IndexMut, Mul};

use crate::{
    vector::Swizzable,
    vectors::{Vector3, Vector4},
    Quaternion, types::{DefaultState, Float},
};

// A simple f32 matrix made of 4 f32/f64 vectors
// TODO: Turn this into a generic struct
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix4x4<T>
where
    T: DefaultState,
{
    pub data: [Vector4<T>; 4],
}

// Default
impl<T> Default for Matrix4x4<T>
where
    T: DefaultState,
{
    fn default() -> Self {
        Self::IDENTITY
    }
}

// Indexer
impl<T> Index<usize> for Matrix4x4<T>
where
    T: DefaultState,
{
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &T {
        let i1 = index / 4;
        let i2 = index % 4;
        let vector = &self.data[i1];
        &vector[i2]
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Matrix4x4<T>
where
    T: DefaultState,
{
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut T {
        let i1 = index / 4;
        let i2 = index % 4;
        let vector = &mut self.data[i1];
        &mut vector[i2]
    }
}

// Identity matrix available for everyone
impl<T> Matrix4x4<T>
where
    T: DefaultState,
{
    // Identity matrix
    pub const IDENTITY: Self = Matrix4x4 {
        data: [Vector4::<T>::X, Vector4::<T>::Y, Vector4::<T>::Z, Vector4::<T>::W],
    };
}

impl<T> Matrix4x4<T>
where
    T: DefaultState,
{
    // Transpose the matrix
    pub fn transpose(&mut self) {
        self.data = self.transposed().data;
    }
    // Return the transpose of this matrix
    pub fn transposed(&self) -> Self {
        let mut output = Self::IDENTITY;
        for x in 0..4 {
            for y in 0..4 {
                let m: &mut T = &mut output[x + y * 4];
                *m = self[y + x * 4];
            }
        }
        output
    }
    // Get the "n" vector
    pub fn get_vec(&self, n: usize) -> &Vector4<T> {
        &self.data[n]
    }
    // Get the "n" vector mutably
    pub fn get_vec_mut(&mut self, n: usize) -> &mut Vector4<T> {
        &mut self.data[n]
    }
}

// Creation code for the matrix
#[allow(dead_code)]
impl<T> Matrix4x4<T>
    where T: DefaultState + Float
{
    // Create a matrix from 4 vector4s
    pub fn new(vec1: Vector4<T>, vec2: Vector4<T>, vec3: Vector4<T>, vec4: Vector4<T>) -> Self {
        Matrix4x4 { data: [vec1, vec2, vec3, vec4] }
    }
    // Create a perspective projection matrix
    // Bonk https://gamedev.stackexchange.com/questions/120338/what-does-a-perspective-projection-matrix-look-like-in-opengl
    pub fn from_perspective(near: T, far: T, aspect: T, fov: T) -> Self {
        // Math
        let first = T::ON / (aspect * (fov / (T::ON * 2.0)).tan());
        let second = T::ON / (fov / (T::ON * 2.0)).tan();
        // The output
        let mut matrix: Self = Self::IDENTITY;
        // Right now it is using row major but I will switch it to collumn major later
        // This is row major
        *matrix.get_vec_mut(0) = Vector4::new(first, 0.0, 0.0, 0.0);
        *matrix.get_vec_mut(1) = Vector4::new(0.0, second, 0.0, 0.0);
        *matrix.get_vec_mut(2) = Vector4::new(0.0, 0.0, -((far + near) / (far - near)), -((2.0 * far * near) / (far - near)));
        *matrix.get_vec_mut(3) = -Vector4::Z;
        matrix.transpose();
        // Transpose the matrix
        matrix
    }
    // Create a translation matrix
    pub fn from_translation(position: Vector3<f32>) -> Self {
        // The output
        let mut matrix: Self = Self::IDENTITY;
        *matrix.get_vec_mut(0) = Vector4::X;
        *matrix.get_vec_mut(1) = Vector4::Y;
        *matrix.get_vec_mut(2) = Vector4::Z;
        *matrix.get_vec_mut(3) = Vector4::new(position[0], position[1], position[2], 1.0);
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
                Vector4::<f32>::new(xaxis.x, xaxis.y, xaxis.z, -xaxis.dot(*eye)),
                Vector4::<f32>::new(yaxis.x, yaxis.y, yaxis.z, -yaxis.dot(*eye)),
                Vector4::<f32>::new(zaxis.x, zaxis.y, zaxis.z, -zaxis.dot(*eye)),
                Vector4::<f32>::W,
            ],
        };

        // Transpose the matrix
        output.transpose();

        output
    }
    // Create a rotation matrix
    pub fn from_quaternion(quat: &Quaternion<T>) -> Self {
        let qx = quat[0];
        let qy = quat[1];
        let qz = quat[2];
        let qw = quat[3];
        let vec1 = Vector4::<T>::new(1.0 - 2.0 * qy * qy - 2.0 * qz * qz, 2.0 * qx * qy - 2.0 * qz * qw, 2.0 * qx * qz + 2.0 * qy * qw, 0.0);
        let vec2 = Vector4::<T>::new(2.0 * qx * qy + 2.0 * qz * qw, 1.0 - 2.0 * qx * qx - 2.0 * qz * qz, 2.0 * qy * qz - 2.0 * qx * qw, 0.0);
        let vec3 = Vector4::<T>::new(2.0 * qx * qz - 2.0 * qy * qw, 2.0 * qy * qz + 2.0 * qx * qw, 1.0 - 2.0 * qx * qx - 2.0 * qy * qy, 0.0);
        let vec4 = Vector4::<T>::W;
        Matrix4x4::new(vec1, vec2, vec3, vec4)
    }
    // Create a scale matrix
    pub fn from_scale(scale: Vector3<T>) -> Self {
        // Too good bro
        Matrix4x4::new(Vector4::X * scale.x, Vector4::Y * scale.y, Vector4::Z * scale.z, Vector4::W)
    }
    // Multiply a matrix by this matrix
    pub fn mul_mat4x4(&self, other: Matrix4x4<T>) -> Self {
        let mut output: Self = Self::IDENTITY;
        // Get the A vectors
        let mut a_vectors: [Vector4<T>; 4] = [Vector4::<T>::ZERO; 4];
        for y in 0..4 {
            a_vectors[y][0] = self.get_vec(0)[y];
            a_vectors[y][1] = self.get_vec(1)[y];
            a_vectors[y][2] = self.get_vec(2)[y];
            a_vectors[y][3] = self.get_vec(3)[y];
        }
        // Get the dot product
        for y in 0..4 {
            for x in 0..4 {
                // Collumn major
                // Y is a
                // X is b

                // Get A
                let a: Vector4<f32> = a_vectors[y];
                let b = *other.get_vec(x);
                let h = &mut output.get_vec_mut(x)[y];
                *h = a.dot(b);
            }
        }
        output
    }
    // Return the inverse of this matrix
    // https://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix/44446912#44446912
    pub fn inverse(&self, inverse: &mut Self) -> bool {
        let m = *self;
        let mut inv = Self::default();

        inv[0] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15] + m[9] * m[7] * m[14] + m[13] * m[6] * m[11] - m[13] * m[7] * m[10];

        inv[4] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15] - m[8] * m[7] * m[14] - m[12] * m[6] * m[11] + m[12] * m[7] * m[10];

        inv[8] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15] + m[8] * m[7] * m[13] + m[12] * m[5] * m[11] - m[12] * m[7] * m[9];

        inv[12] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14] - m[8] * m[6] * m[13] - m[12] * m[5] * m[10] + m[12] * m[6] * m[9];

        inv[1] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15] - m[9] * m[3] * m[14] - m[13] * m[2] * m[11] + m[13] * m[3] * m[10];

        inv[5] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15] + m[8] * m[3] * m[14] + m[12] * m[2] * m[11] - m[12] * m[3] * m[10];

        inv[9] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15] - m[8] * m[3] * m[13] - m[12] * m[1] * m[11] + m[12] * m[3] * m[9];

        inv[13] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14] + m[8] * m[2] * m[13] + m[12] * m[1] * m[10] - m[12] * m[2] * m[9];

        inv[2] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15] + m[5] * m[3] * m[14] + m[13] * m[2] * m[7] - m[13] * m[3] * m[6];

        inv[6] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15] - m[4] * m[3] * m[14] - m[12] * m[2] * m[7] + m[12] * m[3] * m[6];

        inv[10] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15] + m[4] * m[3] * m[13] + m[12] * m[1] * m[7] - m[12] * m[3] * m[5];

        inv[14] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14] - m[4] * m[2] * m[13] - m[12] * m[1] * m[6] + m[12] * m[2] * m[5];

        inv[3] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11] - m[5] * m[3] * m[10] - m[9] * m[2] * m[7] + m[9] * m[3] * m[6];

        inv[7] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11] + m[4] * m[3] * m[10] + m[8] * m[2] * m[7] - m[8] * m[3] * m[6];

        inv[11] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11] - m[4] * m[3] * m[9] - m[8] * m[1] * m[7] + m[8] * m[3] * m[5];

        inv[15] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10] + m[4] * m[2] * m[9] + m[8] * m[1] * m[6] - m[8] * m[2] * m[5];
        let det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];
        // Not valid
        if det == 0.0 {
            return false;
        }

        for i in 0..16 {
            inverse[i] = inv[i] * (1.0 / det);
        }
        true
    }
    // Inversed
    pub fn inversed(&self) -> Self {
        let mut output = Self::IDENTITY;
        self.inverse(&mut output);
        output
    }
}
// Multiply this matrix by another matrix
impl<T> Mul for Matrix4x4<T>
    where T: DefaultState + Float
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        self.mul_mat4x4(rhs)
    }
}
// Transform a vector by the matrix
impl<T> Matrix4x4<T>
    where T: DefaultState + Float
{
    // Transform a 4D vector by the matrix
    pub fn mul_vector(&self, vector: &Vector4<f32>) -> Vector4<f32> {
        // Multiply the vector by this matrix
        let x = self.get_vec(0).dot(*vector);
        let y = self.get_vec(1).dot(*vector);
        let z = self.get_vec(2).dot(*vector);
        let w = self.get_vec(3).dot(*vector);
        Vector4::<f32>::new(x, y, z, w)
    }
    // Transform a 3D point by the matrix, basically create a 4D vector out of it with the W component being 1.0
    pub fn mul_point(&self, point: &Vector3<f32>) -> Vector3<f32> {
        self.mul_vector(&Vector4::new(point.x, point.y, point.z, 1.0)).get3([0, 1, 2])
    }
}
