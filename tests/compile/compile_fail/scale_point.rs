use echo_math::*;

fn main() {
    let p = Point3::new(1.0, 2.0, 3.0);
    let _bad = p * 2.0; // should fail
}
