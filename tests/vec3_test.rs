use echo_math::prelude::*;

#[test]
fn test_vec3_f32_creation() {
    let v = Vec3::<f32>::new(1.0, 2.0, 3.0);
    assert_eq!(v.x, 1.0);
    assert_eq!(v.y, 2.0);
    assert_eq!(v.z, 3.0);
}

#[test]
fn test_vec3_f32_ops() {
    let v1 = Vec3::<f32>::new(1.0, 2.0, 3.0);
    let v2 = Vec3::<f32>::new(4.0, 5.0, 6.0);
    
    let v_add = v1 + v2;
    assert_eq!(v_add.x, 5.0);

    let v_sub = v2 - v1;
    assert_eq!(v_sub.x, 3.0);

    let v_mul = v1 * 2.0;
    assert_eq!(v_mul.x, 2.0);

    let dot = v1.dot(&v2);
    assert_eq!(dot, 32.0);
}

#[test]
fn test_vec3_f32det_creation() {
    let v = Vec3::<F32Det>::new(F32Det(1.0), F32Det(2.0), F32Det(3.0));
    assert_eq!(v.x.0, 1.0);
    assert_eq!(v.y.0, 2.0);
    assert_eq!(v.z.0, 3.0);
}

#[test]
fn test_vec3_f32det_ops() {
    let v1 = Vec3::<F32Det>::new(F32Det(1.0), F32Det(2.0), F32Det(3.0));
    let v2 = Vec3::<F32Det>::new(F32Det(4.0), F32Det(5.0), F32Det(6.0));
    
    let v_add = v1 + v2;
    assert_eq!(v_add.x.0, 5.0);

    let v_sub = v2 - v1;
    assert_eq!(v_sub.x.0, 3.0);

    let v_mul = v1 * F32Det(2.0);
    assert_eq!(v_mul.x.0, 2.0);

    let dot = v1.dot(&v2);
    assert_eq!(dot.0, 32.0);
}
