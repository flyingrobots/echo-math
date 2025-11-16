// This file is expected to fail compilation.
use echo_math::scalar::kinds::DeterministicScalar;

fn deterministic_step<T: DeterministicScalar>(dt: T) {}

fn main() {
    let dt: f32 = 0.016;
    deterministic_step(dt);
}
