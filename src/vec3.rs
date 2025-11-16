use crate::scalar::{RealScalar, Scalar};
use core::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3<T: Scalar> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Scalar> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn splat(v: T) -> Self {
        Self { x: v, y: v, z: v }
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: RealScalar> Vec3<T> {
    pub fn length_sq(&self) -> T {
        self.dot(self)
    }

    pub fn length(&self) -> T {
        self.length_sq().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len == T::ZERO {
            *self
        } else {
            *self * (T::ONE / len)
        }
    }
}

impl<T: Scalar> Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Scalar> Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Scalar> Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}