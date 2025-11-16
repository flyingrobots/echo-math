use crate::scalar::traits::*;

// Placeholder for deterministic f32 wrapper
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct F32Det(pub f32);

impl From<f32> for F32Det {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Scalar for F32Det {
    const ZERO: Self = Self(0.0);
    const ONE: Self = Self(1.0);
    fn epsilon() -> Self { Self(f32::EPSILON) }
    fn from_f32(x: f32) -> Self { Self(x) }
    fn to_f32(self) -> f32 { self.0 }
}

impl RealScalar for F32Det {
    fn abs(self) -> Self { Self(self.0.abs()) }
    fn sqrt(self) -> Self { Self(self.0.sqrt()) } // Not deterministic
    fn rsqrt(self) -> Self { Self(1.0 / self.0.sqrt()) } // Not deterministic
    fn min(self, other: Self) -> Self { Self(self.0.min(other.0)) }
    fn max(self, other: Self) -> Self { Self(self.0.max(other.0)) }
    fn clamp(self, min: Self, max: Self) -> Self { Self(self.0.clamp(min.0, max.0)) }
}

impl TrigScalar for F32Det {
    fn sin(self) -> Self { Self(self.0.sin()) } // Not deterministic
    fn cos(self) -> Self { Self(self.0.cos()) } // Not deterministic
    fn tan(self) -> Self { Self(self.0.tan()) } // Not deterministic
    fn atan2(self, other: Self) -> Self { Self(self.0.atan2(other.0)) } // Not deterministic
    fn asin(self) -> Self { Self(self.0.asin()) } // Not deterministic
    fn acos(self) -> Self { Self(self.0.acos()) } // Not deterministic
}

// Basic ops
use core::ops::{Add, Sub, Mul, Div, Neg};
impl Add for F32Det { type Output = Self; fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) } }
impl Sub for F32Det { type Output = Self; fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) } }
impl Mul for F32Det { type Output = Self; fn mul(self, rhs: Self) -> Self { Self(self.0 * rhs.0) } }
impl Div for F32Det { type Output = Self; fn div(self, rhs: Self) -> Self { Self(self.0 / rhs.0) } }
impl Neg for F32Det { type Output = Self; fn neg(self) -> Self { Self(-self.0) } }