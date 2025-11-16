use crate::scalar::RealScalar;
use crate::vec::vec3::Vec3;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3<T: RealScalar>(pub Vec3<T>);

pub type Point3f = Point3<f32>;

impl<T: RealScalar> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(Vec3::new(x, y, z))
    }
}
