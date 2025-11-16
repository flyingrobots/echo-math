use echo_math::*;

fn step<T: DeterministicScalar>(dt: T) {}

fn main() {
    let dt: f32 = 0.1;
    step(dt); // should fail
}