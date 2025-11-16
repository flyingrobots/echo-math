use echo_math::*;

fn main() {
    let d1 = Direction3::new(1.0, 0.0, 0.0);
    let d2 = Direction3::new(0.0, 1.0, 0.0);

    let _bad = d1 - d2; // should fail
}
