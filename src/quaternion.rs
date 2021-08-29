use std::{
    iter::Copied,
    ops::{Index, IndexMut, Mul},
};

use crate::{types::DefaultStates, Matrix4x4, Swizzable, Vector3, Vector4};

// A quaternion that represents a rotation
#[derive(Debug, Clone, Copy)]
pub struct Quaternion<T> {
    data: Vector4<T>,
}

// Default
impl<T> Default for Quaternion<T>
where
    T: DefaultStates + Clone + Copy + Sized,
{
    fn default() -> Self {
        Self::default_identity()
    }
}

// Indexer
impl<T> Index<usize> for Quaternion<T>
where
    T: DefaultStates + Clone + Copy,
{
    type Output = T;
    // Index
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// Mut indexer
impl<T> IndexMut<usize> for Quaternion<T>
where
    T: DefaultStates + Clone + Copy,
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
where
    T: DefaultStates + Clone + Copy + Sized,
{
    // Defaults
    pub fn default_identity() -> Self {
        Quaternion { data: Vector4::<T>::default_w() }
    }
    // Create a quaternion from euler angles and the order of the angles operation
    // https://www.euclideanspace.com/maths/geometry/rotations/conversions/angleToQuaternion/index.htm
    pub fn from_euler_angles(order: EulerAnglesOrder, euler: Vector3<f32>) -> Quaternion<f32> {
        let output: Quaternion<f32>;
        match order {
            EulerAnglesOrder::XYZ => {
                output = Self::from_x_angle(euler.x())
                    * Self::from_y_angle(euler.y())
                    * Self::from_z_angle(euler.z());
            }
            EulerAnglesOrder::XZY => {
                output = Self::from_x_angle(euler.x())
                    * Self::from_z_angle(euler.z())
                    * Self::from_y_angle(euler.y());
            }
            EulerAnglesOrder::YXZ => {
                output = Self::from_y_angle(euler.y())
                    * Self::from_x_angle(euler.x())
                    * Self::from_z_angle(euler.z());
            }
            EulerAnglesOrder::YZX => {
                output = Self::from_y_angle(euler.y())
                    * Self::from_z_angle(euler.z())
                    * Self::from_x_angle(euler.x());
            }
            EulerAnglesOrder::ZXY => {
                output = Self::from_z_angle(euler.z())
                    * Self::from_x_angle(euler.x())
                    * Self::from_y_angle(euler.y());
            }
            EulerAnglesOrder::ZYX => {
                output = Self::from_z_angle(euler.z())
                    * Self::from_y_angle(euler.y())
                    * Self::from_x_angle(euler.x());
            }
        }
        return output;
    }
    // Create a quaternion from an angle and an axis
    pub fn from_axis_angle(axis: Vector3<f32>, angle: f32) -> Quaternion<f32> {
        // Normalize just in case
        //axis.normalize();
        let x = (angle / 2.0).sin();
        let mut output: Quaternion<f32> = Quaternion::default_identity();
        output[0] = axis.x() * x;
        output[1] = axis.y() * x;
        output[2] = axis.z() * x;
        output[3] = (angle / 2.0).cos();
        /*
        let mut output = Quaternion {
            data: Vector4::<f32>::new(axis.x(), axis.y(), axis.z(), angle)
        };
        */
        return output;
    }
    // Create the quaternion from an angle and the X axis
    pub fn from_x_angle(angle: f32) -> Quaternion<f32> {
        return Self::from_axis_angle(Vector3::default_x(), angle);
    }
    // Create the quaternion from an angle and the Y axis
    pub fn from_y_angle(angle: f32) -> Quaternion<f32> {
        return Self::from_axis_angle(Vector3::default_y(), angle);
    }
    // Create the quaternion from an angle and the Z axis
    pub fn from_z_angle(angle: f32) -> Quaternion<f32> {
        return Self::from_axis_angle(Vector3::default_z(), angle);
    }
}

// Transformations
// https://www.youtube.com/watch?v=Ne3RNhEVSIE&t=451s&ab_channel=JorgeRodriguez
impl Quaternion<f32> {
    // Transform a point by this quaternion
    pub fn mul_point(&self, point: Vector3<f32>) -> Vector3<f32> {
        // Turn the vector into a pure quaternion
        let mut pure: Quaternion<f32> = Quaternion::default_identity();
        let self_vector = self.data.get3([0, 1, 2]);
        pure[3] = 0.0;
        pure[0] = point[0];
        pure[1] = point[1];
        pure[2] = point[2];
        let vector: Vector3<f32> = self_vector.cross(point);
        return point + vector * (2.0 * self[3]) + self_vector.cross(vector) * 2.0;
    }
    // Multiply a quaternion by this quaternion
    pub fn mul_quaternion(&self, other: Quaternion<f32>) -> Quaternion<f32> {
        // The output
        let mut output: Self = Self::default_identity();
        let other_vector = other.data.get3([0, 1, 2]);
        let self_vector = self.data.get3([0, 1, 2]);
        output[3] = self[3] * other[3] + self_vector.dot(other_vector);
        let new_vector: Vector3<f32> = self_vector * other[3] + other_vector * self[3] + self_vector.cross(other_vector);
        output[0] = new_vector.x();
        output[1] = new_vector.y();
        output[2] = new_vector.z();
        return output;
    }
    // Normalize this quaternion
    pub fn normalize(&mut self) {
        self.data.normalize();
    }
}

// Operators
impl Mul for Quaternion<f32> {
    type Output = Quaternion<f32>;

    fn mul(self, rhs: Self) -> Self::Output {
        //return self.mul_quaternion(rhs);
        return rhs.mul_quaternion(self);
    }
}
