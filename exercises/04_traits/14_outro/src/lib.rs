// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SaturatingU16(u16);

//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        Self(value.into())
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        Self(*value)
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        Self((*value).into())
    }
}

//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: SaturatingU16) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = u16;

    fn add(self, rhs: u16) -> Self::Output {
        self.0.saturating_add(rhs)
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;

    fn add(self, rhs: &u16) -> Self::Output {
        Self(self.0.saturating_add(*rhs))
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        &self.0 == other
    }
}
