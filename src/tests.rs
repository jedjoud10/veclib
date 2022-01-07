#[cfg(test)]
mod tests {
    use crate::{
        vector::{Swizzable, UnsizedVector, Vector},
        vectors::{Vector2, Vector3, Vector4},
        Matrix4x4, Quaternion, Vec3Axis,
    };

    // Test if the vector swizzler works
    #[test]
    pub fn swizzle() {
        let y = Vector3::<f32>::new(130.0, 52.0, 86.0);
        assert_eq!(y.get3([0, 0, 0]), Vector3::new(130.0, 130.0, 130.0));
        assert_eq!(y.get3([1, 1, 1]), Vector3::new(52.0, 52.0, 52.0));
        assert_eq!(y.get3([2, 1, 0]), Vector3::new(86.0, 52.0, 130.0));

        let a = Vector3::<bool>::new(false, true, true);
        assert_eq!(a.get3([0, 0, 0]), Vector3::new(false, false, false));
        assert_eq!(a.get3([1, 1, 1]), Vector3::new(true, true, true));
        assert_eq!(a, Vector3::new(false, true, true));
        assert_eq!(a.get4([2, 1, 0, 1]), Vector4::new(true, true, false, true));
    }
    // Test the element wise comparison
    #[test]
    pub fn comparison() {
        let y = Vector3::<f32>::Y;
        let x = Vector3::<f32>::X;
        let element_wise: Vector3<bool> = x.elem_eq(&y);
        assert_eq!(element_wise, Vector3::<bool>::new(false, false, true));
        let element_wise: Vector3<bool> = x.elem_gt(&y);
        assert_eq!(element_wise, Vector3::<bool>::new(true, false, false));
        let element_wise: Vector3<bool> = x.elem_lte(&y);
        assert_eq!(element_wise, Vector3::<bool>::new(false, true, true));
        let booltest = Vector3::<bool>::new(false, true, true) | Vector3::<bool>::new(true, false, true);
        assert_eq!(booltest, Vector3::<bool>::new(true, true, true));

        let val1: Vector2<f32> = Vector2::ONE;
        let val2: Vector2<f32> = Vector2::ZERO;
        assert_eq!(Vector2::<bool>::new(false, true).select(&val1, &val2), Vector2::new(0.0, 1.0));
    }
    // Test the operators
    #[test]
    pub fn operators() {
        let y = Vector3::<f32>::Y;
        let test = y + Vector3::<f32>::ONE;
        assert_eq!(test, Vector3::<f32>::new(1.0, 2.0, 1.0));
        let y = Vector3::<f32>::Y;
        let x = Vector3::<f32>::X;
        let addition = x + y;
        assert_eq!(addition, Vector3::new(1.0, 1.0, 0.0));
        assert_eq!(Vector4::<i32>::ONE - Vector4::<i32>::W, Vector4::new(1, 1, 1, 0));
        assert_eq!(Vector4::<i32>::ONE * Vector4::<i32>::W * 2, Vector4::new(0, 0, 0, 2));
        assert_eq!(Vector4::<i32>::ONE + Vector4::<i32>::W * 2, Vector4::new(1, 1, 1, 3));
        assert_eq!(Vector2::<i32>::ONE - Vector2::<i32>::ZERO, Vector2::ONE);
        assert_eq!(-Vector3::<i32>::ONE, Vector3::new(-1, -1, -1));
        assert_eq!(-Vector3::<i32>::ONE + Vector3::<i32>::Y * 2, Vector3::new(-1, 1, -1));
        assert_ne!(-Vector2::<f32>::ONE, Vector2::<f32>::ONE);
        assert_ne!(-Vector2::<f64>::ONE, Vector2::<f64>::ONE);
    }
    // Vector operations
    #[test]
    pub fn operations() {
        let val = Vector3::<f32>::X;
        let test = val.dot(Vector3::Y);
        assert_eq!(test, 0.0);
        let k = Vector3::<f32>::new(2.0, 3.0, 4.0).cross(Vector3::<f32>::new(5.0, 6.0, 7.0));
        assert_eq!(k, Vector3::<f32>::new(-3.0, 6.0, -3.0));
        assert_eq!(Vector2::ZERO, Vector2::new(0.0, 0.0));
    }
    // Matrix multiplication
    #[test]
    pub fn matrix() {
        // Non inversible matrix
        let mat1 = Matrix4x4::<f32>::new(
            Vector4::new(5.0, 5.0, 2.0, 1.0),
            Vector4::new(2.0, -4.0, -2.0, -3.0),
            Vector4::new(9.0, 5.0, -11.0, -2.0),
            Vector4::new(5.0, 5.0, 5.0, -2.0),
        );
        let _mat2 = Matrix4x4::<f32>::new(Vector4::new(2.0, 1.0, 1.0, 1.0), Vector4::ONE, Vector4::new(4.0, 1.0, 1.0, 3.0), Vector4::ONE);
        let _q = Matrix4x4::<f32> {
            data: [
                Vector4::new(0.0, 1.0, 2.0, 3.0),
                Vector4::new(4.0, 5.0, 6.0, 7.0),
                Vector4::new(8.0, 9.0, 10.0, 11.0),
                Vector4::new(12.0, 13.0, 14.0, 15.0),
            ],
        };
        assert_eq!(mat1.get_vec(1).x, mat1[4]);
        assert_eq!(mat1.get_vec(1).y, mat1[5]);
        assert_eq!(mat1.get_vec(1).z, mat1[6]);
        assert_eq!(Matrix4x4::IDENTITY * Matrix4x4::IDENTITY, Matrix4x4::<f32>::IDENTITY);
        assert_eq!(Matrix4x4::<f32>::IDENTITY.transposed(), Matrix4x4::<f32>::IDENTITY.transposed());
        assert_eq!(
            Matrix4x4::<f32>::from_scale(Vector3::<f32>::ONE * 100.0).mul_point(&Vector3::<f32>::ONE),
            Vector3::<f32>::ONE * 100.0
        );
        assert_eq!(Matrix4x4::<f32>::IDENTITY, Matrix4x4::<f32>::IDENTITY.inversed());
    }
    // Quaternion tests
    #[test]
    pub fn quaternion() {
        let quaternion = Quaternion::<f32>::from_y_angle(90_f32.to_radians());
        let quaternion2 = Quaternion::<f32>::from_x_angle(45_f32.to_radians());
        println!("{:?}", quaternion);
        println!("{:?}", quaternion2 * quaternion);
        println!("{:?}", Quaternion::<f32>::IDENTITY.mul_point(Vector3::<f32>::Y));
        //assert_eq!(quaternion.mul_point(Vector3::<f32>::new(1.0, 0.0, 0.0)), Vector3::default_z());
    }
    // Vector axis
    #[test]
    pub fn vector_axis() {
        let v1 = Vector3::<f32>::new(10.0, -5.0, 20.0);
        assert_eq!(v1.get_axis(Vec3Axis::X), 10.0);
        assert_eq!(v1.get_axis(Vec3Axis::Y), -5.0);
        assert_eq!(v1.get_axis(Vec3Axis::Z), 20.0);
    }
    // Test the unsized vector
    #[test]
    pub fn unsized_vector() {
        let scalar = 0.0_f64;
        let v1 = Vector3::<f64>::new(10.0, -5.0, 20.0);
        let u1 = scalar.get_unsized();
        let u2 = v1.get_unsized();
        assert_eq!(u1, UnsizedVector::Scalar(0.0));
        assert_eq!(u2, UnsizedVector::Vec3(Vector3::<f64>::new(10.0, -5.0, 20.0)));
    }
}
