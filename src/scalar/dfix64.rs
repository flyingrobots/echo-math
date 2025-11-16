use crate::scalar::traits::*;

// Placeholder for deterministic fixed-point 64-bit
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct DFix64(pub i64);

const FIXED_POINT_SHIFT: i64 = 16;
const FIXED_POINT_ONE: i64 = 1 << FIXED_POINT_SHIFT;

impl From<i64> for DFix64 {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Scalar for DFix64 {
    const ZERO: Self = Self(0);
    const ONE: Self = Self(FIXED_POINT_ONE);
    fn epsilon() -> Self { Self(1) }
    fn from_f32(x: f32) -> Self { Self((x * (FIXED_POINT_ONE as f32)) as i64) }
    fn to_f32(self) -> f32 { (self.0 as f32) / (FIXED_POINT_ONE as f32) }
}

impl RealScalar for DFix64 {
    fn abs(self) -> Self { Self(self.0.abs()) }
    fn sqrt(self) -> Self { Self(0) } // Placeholder
    fn rsqrt(self) -> Self { Self(0) } // Placeholder
    fn min(self, other: Self) -> Self { Self(self.0.min(other.0)) }
    fn max(self, other: Self) -> Self { Self(self.0.max(other.0)) }
    fn clamp(self, min: Self, max: Self) -> Self { Self(self.0.clamp(min.0, max.0)) }
}

impl TrigScalar for DFix64 {
    fn sin(self) -> Self { Self(0) } // Placeholder
    fn cos(self) -> Self { Self(0) } // Placeholder
    fn tan(self) -> Self { Self(0) } // Placeholder
    fn atan2(self, _other: Self) -> Self { Self(0) } // Placeholder
    fn asin(self) -> Self { Self(0) } // Placeholder
    fn acos(self) -> Self { Self(0) } // Placeholder
}

// Basic ops
use core::ops::{Add, Sub, Mul, Div, Neg};
impl Add for DFix64 { type Output = Self; fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) } }
impl Sub for DFix64 { type Output = Self; fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) } }
impl Mul for DFix64 { type Output = Self; fn mul(self, rhs: Self) -> Self { Self((self.0 * rhs.0) >> FIXED_POINT_SHIFT) } }
impl Div for DFix64 { type Output = Self; fn div(self, rhs: Self) -> Self { Self((self.0 << FIXED_POINT_SHIFT) / rhs.0) } }
impl Neg for DFix64 { type Output = Self; fn neg(self) -> Self { Self(-self.0) } }