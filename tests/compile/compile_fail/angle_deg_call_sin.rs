// This file is expected to fail compilation.
use echo_math::angle::DegAngle;

fn main() {
    let a = DegAngle::<f32>::from_degrees(45.0);
    let s = a.sin();
}
