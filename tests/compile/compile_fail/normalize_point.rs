use echo_math::*;

fn main() {
    let p = Point3::new(1.0, 2.0, 3.0);
    p.normalize(); // should fail, normalize doesn't exist
}
