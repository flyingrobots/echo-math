use super::traits::*;

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
