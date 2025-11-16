// This file is expected to fail compilation.
use echo_math::vec::point3::Point3f;

fn main() {
    let p1 = Point3f::new(1.0, 2.0, 3.0);
    let p2 = Point3f::new(4.0, 5.0, 6.0);

    let _bad = p1 + p2;
}
