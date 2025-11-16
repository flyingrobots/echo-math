use core::ops::{
    Add, Sub, Mul, Div, Neg,
};

pub(crate) mod sealed {
    pub trait Sealed {}
}

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

pub trait DeterministicScalar: TrigScalar {}
pub trait NondetScalar: TrigScalar {}

// Implement sealed for our types
impl sealed::Sealed for f32 {}
impl sealed::Sealed for F32Det {}
impl sealed::Sealed for DFix64 {}

// f32 implementations
impl Scalar for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;

    fn epsilon() -> Self { f32::EPSILON }

    fn from_f32(x: f32) -> Self { x }
    fn to_f32(self) -> f32 { self }
}

impl RealScalar for f32 {
    fn abs(self) -> Self { f32::abs(self) }
    fn sqrt(self) -> Self { f32::sqrt(self) }
    fn rsqrt(self) -> Self { 1.0 / f32::sqrt(self) }
    fn min(self, other: Self) -> Self { f32::min(self, other) }
    fn max(self, other: Self) -> Self { f32::max(self, other) }
    fn clamp(self, min: Self, max: Self) -> Self { f32::clamp(self, min, max) }
}

impl TrigScalar for f32 {
    fn sin(self) -> Self { f32::sin(self) }
    fn cos(self) -> Self { f32::cos(self) }
    fn tan(self) -> Self { f32::tan(self) }
    fn atan2(self, other: Self) -> Self { f32::atan2(self, other) }
    fn asin(self) -> Self { f32::asin(self) }
    fn acos(self) -> Self { f32::acos(self) }
}

impl NondetScalar for f32 {}

// F32Det implementation
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Default)]
pub struct F32Det(pub f32);

impl From<f32> for F32Det {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl Add for F32Det { type Output = Self; fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) } }
impl Sub for F32Det { type Output = Self; fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) } }
impl Mul for F32Det { type Output = Self; fn mul(self, rhs: Self) -> Self { Self(self.0 * rhs.0) } }
impl Div for F32Det { type Output = Self; fn div(self, rhs: Self) -> Self { Self(self.0 / rhs.0) } }
impl Neg for F32Det { type Output = Self; fn neg(self) -> Self { Self(-self.0) } }

impl Scalar for F32Det {
    const ZERO: Self = Self(0.0);
    const ONE: Self = Self(1.0);
    fn epsilon() -> Self { Self(f32::EPSILON) }
    fn from_f32(x: f32) -> Self { Self(x) }
    fn to_f32(self) -> f32 { self.0 }
}

impl RealScalar for F32Det {
    fn abs(self) -> Self { Self(self.0.abs()) }
    fn sqrt(self) -> Self { Self(self.0.sqrt()) }
    fn rsqrt(self) -> Self { Self(1.0 / self.0.sqrt()) }
    fn min(self, other: Self) -> Self { Self(self.0.min(other.0)) }
    fn max(self, other: Self) -> Self { Self(self.0.max(other.0)) }
    fn clamp(self, min: Self, max: Self) -> Self { Self(self.0.clamp(min.0, max.0)) }
}

impl TrigScalar for F32Det {
    fn sin(self) -> Self { Self(self.0.sin()) }
    fn cos(self) -> Self { Self(self.0.cos()) }
    fn tan(self) -> Self { Self(self.0.tan()) }
    fn atan2(self, other: Self) -> Self { Self(self.0.atan2(other.0)) }
    fn asin(self) -> Self { Self(self.0.asin()) }
    fn acos(self) -> Self { Self(self.0.acos()) }
}

impl DeterministicScalar for F32Det {}

// DFix64 implementation
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct DFix64(pub i64);

const FIXED_POINT_SHIFT: i64 = 16;
const FIXED_POINT_ONE: i64 = 1 << FIXED_POINT_SHIFT;

impl From<i64> for DFix64 {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl Add for DFix64 { type Output = Self; fn add(self, rhs: Self) -> Self { Self(self.0 + rhs.0) } }
impl Sub for DFix64 { type Output = Self; fn sub(self, rhs: Self) -> Self { Self(self.0 - rhs.0) } }
impl Mul for DFix64 { type Output = Self; fn mul(self, rhs: Self) -> Self { Self((self.0 * rhs.0) >> FIXED_POINT_SHIFT) } }
impl Div for DFix64 { type Output = Self; fn div(self, rhs: Self) -> Self { Self((self.0 << FIXED_POINT_SHIFT) / rhs.0) } }
impl Neg for DFix64 { type Output = Self; fn neg(self) -> Self { Self(-self.0) } }

impl Scalar for DFix64 {
    const ZERO: Self = Self(0);
    const ONE: Self = Self(FIXED_POINT_ONE);
    fn epsilon() -> Self { Self(1) }
    fn from_f32(x: f32) -> Self { Self((x * (FIXED_POINT_ONE as f32)) as i64) }
    fn to_f32(self) -> f32 { (self.0 as f32) / (FIXED_POINT_ONE as f32) }
}

impl RealScalar for DFix64 {
    fn abs(self) -> Self { Self(self.0.abs()) }
    fn sqrt(self) -> Self { Self(0) } // Placeholder for fixed-point sqrt
    fn rsqrt(self) -> Self { Self(0) } // Placeholder for fixed-point rsqrt
    fn min(self, other: Self) -> Self { Self(self.0.min(other.0)) }
    fn max(self, other: Self) -> Self { Self(self.0.max(other.0)) }
    fn clamp(self, min: Self, max: Self) -> Self { Self(self.0.clamp(min.0, max.0)) }
}

impl TrigScalar for DFix64 {
    fn sin(self) -> Self { Self(0) } // Placeholder for fixed-point sin
    fn cos(self) -> Self { Self(0) } // Placeholder for fixed-point cos
    fn tan(self) -> Self { Self(0) } // Placeholder for fixed-point tan
    fn atan2(self, _other: Self) -> Self { Self(0) } // Placeholder for fixed-point atan2
    fn asin(self) -> Self { Self(0) } // Placeholder for fixed-point asin
    fn acos(self) -> Self { Self(0) } // Placeholder for fixed-point acos
}

impl DeterministicScalar for DFix64 {}
