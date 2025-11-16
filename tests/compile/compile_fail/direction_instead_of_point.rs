use echo_math::*;

fn set_position<T: Scalar>(p: Point3<T>) {}

fn main() {
    let d = Direction3::new(1.0, 1.0, 1.0);
    set_position(d); // should fail
}
