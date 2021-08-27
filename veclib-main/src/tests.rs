#[cfg(test)]
mod tests {
    use crate::{vector::Swizzable, vectors::{Vector2, Vector3, Vector4}};

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
    }
    // Vector arithemtics
    #[test]
    pub fn arithemtics() {
        let val = Vector3::<f32>::X;
        let test = val.dot(&Vector3::Y);
        assert_eq!(test, 0.0);
    }
}