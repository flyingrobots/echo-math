use crate::{Scalar, Vec3};
use core::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point3<T: Scalar>(pub Vec3<T>);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Direction3<T: Scalar>(pub Vec3<T>);

impl<T: Scalar> Point3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Point3(Vec3::new(x, y, z))
    }
}

impl<T: Scalar> Direction3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Direction3(Vec3::new(x, y, z))
    }
}

// Operators for Point3 and Direction3
impl<T: Scalar> Add<Direction3<T>> for Point3<T> {
    type Output = Point3<T>;
    fn add(self, dir: Direction3<T>) -> Self::Output {
        Point3(Vec3 {
            x: self.0.x + dir.0.x,
            y: self.0.y + dir.0.y,
            z: self.0.z + dir.0.z,
        })
    }
}

impl<T: Scalar> Sub for Point3<T> {
    type Output = Direction3<T>;
    fn sub(self, rhs: Point3<T>) -> Self::Output {
        Direction3(Vec3 {
            x: self.0.x - rhs.0.x,
            y: self.0.y - rhs.0.y,
            z: self.0.z - rhs.0.z,
        })
    }
}

impl<T: Scalar> Mul<T> for Direction3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Direction3(self.0 * rhs)
    }
}