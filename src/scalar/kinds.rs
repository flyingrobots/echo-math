use super::traits::TrigScalar;

pub trait DeterministicScalar: TrigScalar {}
pub trait NondetScalar: TrigScalar {}

// implementations
// These will be fleshed out later.
use crate::scalar::f32_det::F32Det;
use crate::scalar::dfix64::DFix64;

impl DeterministicScalar for F32Det {}
impl DeterministicScalar for DFix64 {}

impl NondetScalar for f32 {}
