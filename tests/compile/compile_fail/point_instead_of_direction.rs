use echo_math::*;

fn move_dir<T: Scalar>(dir: Direction3<T>) {}

fn main() {
    let p = Point3::new(1.0, 2.0, 3.0);
    move_dir(p); // should fail
}
