use echo_math::*;

struct Ray<T: Scalar> {
    origin: Point3<T>,
    dir: Direction3<T>,
}

fn main() {
    let origin = Direction3::new(0.0, 0.0, 0.0); // wrong
    let dir = Point3::new(1.0, 0.0, 1.0);        // wrong

    let _ = Ray { origin, dir }; // should fail
}
