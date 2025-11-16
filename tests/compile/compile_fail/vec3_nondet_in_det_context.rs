use echo_math::*;

fn integrate<T: DeterministicScalar>(v: Vec3<T>) {}

fn main() {
    let v = Vec3::<f32>::new(1.0, 2.0, 3.0);
    integrate(v); // should fail
}
