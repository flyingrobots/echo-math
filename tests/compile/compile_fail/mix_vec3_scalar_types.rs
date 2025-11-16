use echo_math::*;

fn main() {
    let a = Vec3::<F32Det>::new(F32Det(1.0), F32Det(2.0), F32Det(3.0));
    let b = Vec3::<f32>::new(1.0, 2.0, 3.0);

    let _c = a + b; // should fail; no Add impl
}
