use std::ops::{Index, IndexMut, Mul};

use crate::{Swizzable, Vector3, Vector4, types::{DefaultState, Float}};

// A quaternion that represents a rotation
#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T> 
    where T: DefaultState
{
    data: Vector4<T>,
}

// Default
impl<T> Default for Quaternion<T>
    where T: DefaultState,
{
    fn default() -> Self {
        Self::IDENTITY
    }
}

// Indexer
impl<T> Index<usize> for Quaternion<T>
    where T: DefaultState,
{
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Quaternion<T>
    where T: DefaultState,
{
    // Mut index
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

// The order of the angles operations when creating a Quaternion from euler angles
pub enum EulerAnglesOrder {
    XYZ,
    XZY,
    YXZ,
    YZX,
    ZXY,
    ZYX,
}

// Da code
impl<T> Quaternion<T>
    where T: DefaultState + Float,
{
    // Identity
    pub const IDENTITY: Self = Self { data: Vector4::<T>::W };
    // Create a quaternion from euler angles and the order of the angles operation
    // https://www.euclideanspace.com/maths/geometry/rotations/conversions/angleToQuaternion/index.htm
    pub fn from_euler_angles(order: EulerAnglesOrder, euler: Vector3<T>) -> Quaternion<T> {
        let output: Quaternion<T>;
        match order {
            EulerAnglesOrder::XYZ => {
                output = Self::from_z_angle(euler.z) * Self::from_y_angle(euler.y) * Self::from_x_angle(euler.x);
            }
            EulerAnglesOrder::XZY => {
                output = Self::from_y_angle(euler.y) * Self::from_z_angle(euler.z) * Self::from_x_angle(euler.x);
            }
            EulerAnglesOrder::YXZ => {
                output = Self::from_z_angle(euler.z) * Self::from_x_angle(euler.x) * Self::from_y_angle(euler.y);
            }
            EulerAnglesOrder::YZX => {
                output = Self::from_x_angle(euler.x) * Self::from_z_angle(euler.z) * Self::from_y_angle(euler.y);
            }
            EulerAnglesOrder::ZXY => {
                output = Self::from_y_angle(euler.y) * Self::from_x_angle(euler.x) * Self::from_z_angle(euler.z);
            }
            EulerAnglesOrder::ZYX => {
                output = Self::from_x_angle(euler.x) * Self::from_y_angle(euler.y) * Self::from_z_angle(euler.z);
            }
        }
        output
    }
    // Create a quaternion from an angle and an axis
    pub fn from_axis_angle(axis: Vector3<T>, angle: T) -> Quaternion<T> {
        // Normalize just in case
        //axis.normalize();
        let x = (angle / 2.0).sin();
        let mut output: Quaternion<T> = Quaternion::IDENTITY;
        output[0] = axis.x * x;
        output[1] = axis.y * x;
        output[2] = axis.z * x;
        output[3] = (angle / 2.0).cos();
        /*
        let mut output = Quaternion {
            data: Vector4::<f32>::new(axis.x, axis.y, axis.z, angle)
        };
        */
        output
    }
    // Create the quaternion from an angle and the X axis
    pub fn from_x_angle(angle: T) -> Quaternion<T> {
        Self::from_axis_angle(Vector3::X, angle)
    }
    // Create the quaternion from an angle and the Y axis
    pub fn from_y_angle(angle: T) -> Quaternion<T> {
        Self::from_axis_angle(Vector3::Y, angle)
    }
    // Create the quaternion from an angle and the Z axis
    pub fn from_z_angle(angle: T) -> Quaternion<T> {
        Self::from_axis_angle(Vector3::Z, angle)
    }
}

// Transformations
// https://www.youtube.com/watch?v=Ne3RNhEVSIE&t=451s&ab_channel=JorgeRodriguez
impl<T> Quaternion<T>
    where T: DefaultState + Float
{
    // Transform a point by this quaternion
    pub fn mul_point(&self, point: Vector3<T>) -> Vector3<T> {
        // Turn the vector into a pure quaternion
        let mut pure: Quaternion<T> = Quaternion::IDENTITY;
        let self_vector = self.data.get3([0, 1, 2]);
        pure[3] = 0.0;
        pure[0] = point[0];
        pure[1] = point[1];
        pure[2] = point[2];
        let vector: Vector3<f32> = self_vector.cross(point);
        point + vector * (2.0 * self[3]) + self_vector.cross(vector) * 2.0
    }
    // Multiply a quaternion by this quaternion
    pub fn mul_quaternion(&self, other: Quaternion<T>) -> Quaternion<T> {
        // The output
        let mut output: Self = Self::IDENTITY;
        let other_vector = other.data.get3([0, 1, 2]);
        let self_vector = self.data.get3([0, 1, 2]);
        output[3] = self[3] * other[3] + self_vector.dot(other_vector);
        let new_vector: Vector3<f32> = self_vector * other[3] + other_vector * self[3] + self_vector.cross(other_vector);
        output[0] = new_vector.x;
        output[1] = new_vector.y;
        output[2] = new_vector.z;
        output
    }
    // Normalize this quaternion
    pub fn normalize(&mut self) {
        self.data.normalize();
    }
}

// Operators
impl<T> Mul for Quaternion<T>
    where T: DefaultState + Float
{
    type Output = Quaternion<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        //return self.mul_quaternion(rhs);
        rhs.mul_quaternion(self)
    }
}
