use core::marker::PhantomData;
use crate::scalar::TrigScalar;

pub enum Rad {}
pub enum Deg {}

pub struct Angle<T: TrigScalar, U> {
    value: T,
    _unit: PhantomData<U>,
}

pub type RadAngle<T> = Angle<T, Rad>;
pub type DegAngle<T> = Angle<T, Deg>;

impl<T: TrigScalar> RadAngle<T> {
    pub fn from_radians(r: T) -> Self {
        Self { value: r, _unit: PhantomData }
    }

    pub fn to_radians(self) -> T { self.value }

    pub fn sin(self) -> T { self.value.sin() }
    pub fn cos(self) -> T { self.value.cos() }
    pub fn tan(self) -> T { self.value.tan() }
}

impl<T: TrigScalar> DegAngle<T> {
    pub fn from_degrees(d: T) -> Self {
        Self { value: d, _unit: PhantomData }
    }

    pub fn to_degrees(self) -> T { self.value }

    pub fn to_radians(self) -> RadAngle<T> {
        // 180 / π as a T
        let k = T::from_f32(core::f32::consts::PI / 180.0);
        RadAngle::from_radians(self.value * k)
    }
}