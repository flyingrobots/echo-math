use core::ops::{
    Add, Sub, Mul, Div, Neg,
};

pub(crate) mod sealed {
    pub trait Sealed {}
}

impl sealed::Sealed for f32 {}
impl sealed::Sealed for super::f32_det::F32Det {}
impl sealed::Sealed for super::dfix64::DFix64 {}


/// Minimal field-like scalar: add/mul/div, constants, conversions.
pub trait Scalar:
    Copy + Clone + PartialEq + PartialOrd +
    Add<Output = Self> + Sub<Output = Self> +
    Mul<Output = Self> + Div<Output = Self> +
    Neg<Output = Self> +
    Sized + 'static +
    sealed::Sealed
{
    const ZERO: Self;
    const ONE: Self;

    /// Smallest distinguishable step you care about.
    fn epsilon() -> Self;

    fn from_f32(x: f32) -> Self;
    fn to_f32(self) -> f32;
}

/// Scalars that support "real" operations like sqrt.
pub trait RealScalar: Scalar {
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn rsqrt(self) -> Self;
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    fn clamp(self, min: Self, max: Self) -> Self;
}

/// Scalars that support trig.
pub trait TrigScalar: RealScalar {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
}
