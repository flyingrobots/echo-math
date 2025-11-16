// echo-math/src/lib.rs

#![allow(dead_code)] // TODO: Remove this once the library is fleshed out.

pub mod scalar;
pub mod angle;
pub mod vec;
// pub mod quat;
// pub mod mat4;
// pub mod transform;
// pub mod ops;

// Prelude for easy importing of common types.
pub mod prelude {
    pub use crate::scalar::{Scalar, RealScalar, TrigScalar};
    pub use crate::scalar::{DeterministicScalar, NondetScalar};
    pub use crate::scalar::f32_det::F32Det;
    pub use crate::angle::{Angle, RadAngle, DegAngle};
    pub use crate::vec::vec3::Vec3;
    pub use crate::vec::point3::{Point3, Point3f};
    // ... other common types
}

