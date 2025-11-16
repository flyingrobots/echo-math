use echo_math::*;

fn main() {
    let p1 = Point3::new(1.0, 2.0, 3.0);
    let p2 = Point3::new(4.0, 5.0, 6.0);

    let _bad = p1 + p2; // should fail
}