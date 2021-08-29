#[cfg(test)]
mod tests {
    use crate::{Matrix4x4, Quaternion, vector::Swizzable, vectors::{Vector2, Vector3, Vector4}};

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
        let y = Vector3::<f32>::default_y();
        let x = Vector3::<f32>::default_x();
        let element_wise: Vector3<bool> = x.elem_eq(&y);
        assert_eq!(element_wise, Vector3::<bool>::new(false, false, true));
        let element_wise: Vector3<bool> = x.elem_gt(&y);
        assert_eq!(element_wise, Vector3::<bool>::new(true, false, false));
        let element_wise: Vector3<bool> = x.elem_lte(&y);
        assert_eq!(element_wise, Vector3::<bool>::new(false, true, true));
    }
    // Test the operators
    #[test]
    pub fn operators() {
        let y = Vector3::<f32>::default_y();
        let test = y + Vector3::<f32>::default_one();
        assert_eq!(test, Vector3::<f32>::new(1.0, 2.0, 1.0));
        let y = Vector3::<f32>::default_y();
        let x = Vector3::<f32>::default_x();
        let addition = x + y;
        assert_eq!(addition, Vector3::new(1.0, 1.0, 0.0));
        assert_eq!(Vector4::<i32>::default_one() - Vector4::<i32>::default_w(), Vector4::new(1, 1, 1, 0));
        assert_eq!(Vector4::<i32>::default_one() * Vector4::<i32>::default_w() * 2, Vector4::new(0, 0, 0, 2));
        assert_eq!(Vector4::<i32>::default_one() + Vector4::<i32>::default_w() * 2, Vector4::new(1, 1, 1, 3));
        assert_eq!(Vector2::<i32>::default_one() - Vector2::<i32>::default_zero(), Vector2::default_one());
        assert_eq!(-Vector3::<i32>::default_one(), Vector3::new(-1, -1, -1));
        assert_eq!(-Vector3::<i32>::default_one() + Vector3::<i32>::default_y() * 2, Vector3::new(-1, 1, -1));
        assert_ne!(-Vector2::<f32>::default_one(), Vector2::<f32>::default_one());
        assert_ne!(-Vector2::<f64>::default_one(), Vector2::<f64>::default_one());
    }
    // Vector arithemtics
    #[test]
    pub fn arithemtics() {
        let val = Vector3::<f32>::default_x();
        let test = val.dot(Vector3::default_y());
        assert_eq!(test, 0.0);
        let k = Vector3::<f32>::new(2.0, 3.0, 4.0).cross(Vector3::<f32>::new(5.0, 6.0, 7.0));
        assert_eq!(k, Vector3::<f32>::new(-3.0, 6.0, -3.0));
    }
    // Matrix multiplication
    #[test]
    pub fn matrix() {
        let mat1 = Matrix4x4::<f32>::new(
            Vector4::default_one(),
            Vector4::new(1.0, 5.0, 5.0, 1.0),
            Vector4::new(1.0, 5.0, 5.0, 1.0),
            Vector4::default_one(),
        );
        let mat2 = Matrix4x4::<f32>::new(
            Vector4::new(2.0, 1.0, 1.0, 1.0),
            Vector4::default_one(),
            Vector4::new(4.0, 1.0, 1.0, 3.0),
            Vector4::default_one(),
        );
        let q = Matrix4x4::<f32> {
            data: [Vector4::new(0.0, 1.0, 2.0, 3.0), Vector4::new(4.0, 5.0, 6.0, 7.0), Vector4::new(8.0, 9.0, 10.0, 11.0), Vector4::new(12.0, 13.0, 14.0, 15.0)]
        };
        println!("{:?}", q.transposed());
        assert_eq!(Matrix4x4::default_identity() * Matrix4x4::default_identity(), Matrix4x4::<f32>::default_identity());
        assert_eq!(Matrix4x4::<f32>::default_identity().transposed(), Matrix4x4::<f32>::default_identity().transposed());
        assert_eq!(Matrix4x4::<f32>::from_scale(Vector3::<f32>::default_one() * 100.0).mul_point(&Vector3::<f32>::default_one()), Vector3::<f32>::default_one() * 100.0);
    }
    // Quaternion tests
    #[test]
    pub fn quaternion() {
        let quaternion = Quaternion::<f32>::from_y_angle(90_f32.to_radians());
        let quaternion2 = Quaternion::<f32>::from_x_angle(45_f32.to_radians());        
        println!("{:?}", quaternion);
        println!("{:?}", quaternion2 * quaternion);
        println!("{:?}", Quaternion::<f32>::default_identity().mul_point(Vector3::<f32>::default_y()));
        //assert_eq!(quaternion.mul_point(Vector3::<f32>::new(1.0, 0.0, 0.0)), Vector3::default_z());
    }
}
