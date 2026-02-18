#![cfg_attr(not(test), no_std)]

#![allow(unused)]

pub(crate) use core::marker::PhantomData;
pub(crate) use core::ops::*;
pub(crate) use typenum::*;
use bitsong::*;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[derive(zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes)]
#[derive(SongSize, ToSong, FromSong)]
#[serde(transparent)]
pub struct Quantity<T, D: Dimension + ?Sized> {
    pub value: T,
    pub dim: PhantomData<D>,
}

impl <T: core::fmt::Debug, D: Dimension + ?Sized> core::fmt::Debug for Quantity<T, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)?;
        f.write_str(" ")?;
        core::fmt::Debug::fmt(&D::default(), f)?;
        Ok(())
    }
}

impl <T: core::fmt::Display, D: Dimension + ?Sized> core::fmt::Display for Quantity<T, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)?;
        f.write_str(" ")?;
        core::fmt::Display::fmt(&D::default(), f)?;
        Ok(())
    }
}

impl <T, D: Dimension + ?Sized> Quantity<T, D> {
    pub fn new(value: T) -> Self {
        Quantity { value, dim: PhantomData }
    }
}

impl <T, D: Dimension + ?Sized> Deref for Quantity<T, D> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl <T, D: Dimension + ?Sized> DerefMut for Quantity<T, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub trait FromUnits<T> {
    fn from_units(value: &T) -> Self;
}

pub trait IntoUnits<U> {
    fn into_units(&self) -> U;
}

impl <T, U: FromUnits<T>> IntoUnits<U> for T {
    fn into_units(&self) -> U {
        <U as FromUnits<T>>::from_units(self)
    }
}

impl<T: Add<T, Output = T>, D: Dimension> Add<Quantity<T, D>> for Quantity<T, D> {
    type Output = Self;
    fn add(self, rhs: Quantity<T, D>) -> Self::Output {
        Quantity::new(self.value.add(rhs.value))
    }
}

impl<T: AddAssign<T>, D: Dimension> AddAssign<Quantity<T, D>> for Quantity<T, D> {
    fn add_assign(&mut self, rhs: Quantity<T, D>) {
        self.value.add_assign(rhs.value);
    }
}

impl<T: Sub<T, Output = T>, D: Dimension> Sub<Quantity<T, D>> for Quantity<T, D> {
    type Output = Self;
    fn sub(self, rhs: Quantity<T, D>) -> Self::Output {
        Quantity::new(self.value.sub(rhs.value))
    }
}

impl<T: SubAssign<T>, D: Dimension> SubAssign<Quantity<T, D>> for Quantity<T, D> {
    fn sub_assign(&mut self, rhs: Quantity<T, D>) {
        self.value.sub_assign(rhs.value);
    }
}

impl<T: Mul<T, Output = T>, A: Dimension + Mul<B>, B: Dimension> Mul<Quantity<T, B>> for Quantity<T, A>
where <A as Mul<B>>::Output: Dimension {
    type Output = Quantity<T, <A as Mul<B>>::Output>;
    fn mul(self, rhs: Quantity<T, B>) -> Self::Output {
        Quantity::new(self.value.mul(rhs.value))
    }
}

impl<T: Div<T, Output = T>, A: Dimension + Div<B>, B: Dimension> Div<Quantity<T, B>> for Quantity<T, A>
where <A as Div<B>>::Output: Dimension {
    type Output = Quantity<T, <A as Div<B>>::Output>;
    fn div(self, rhs: Quantity<T, B>) -> Self::Output {
        Quantity::new(self.value.div(rhs.value))
    }
}

pub type Multiply<A, B> = <A as Mul<B>>::Output;
pub type Divide<N, D> = <N as Div<D>>::Output;

pub type MetersPerSecond<T> = Quantity<T, Divide<UnitMeters, UnitSeconds>>;
pub type MetersPerSecond2<T> = Quantity<T, Divide<Divide<UnitMeters, UnitSeconds>, UnitSeconds>>;
pub type RadiansPerSecond<T> = Quantity<T, Divide<UnitRadians, UnitSeconds>>;
pub type RadiansPerSecond2<T> = Quantity<T, Divide<Divide<UnitRadians, UnitSeconds>, UnitSeconds>>;
pub type DegreesPerSecond<T> = Quantity<T, Divide<UnitDegrees, UnitSeconds>>;
pub type DegreesPerSecond2<T> = Quantity<T, Divide<Divide<UnitDegrees, UnitSeconds>, UnitSeconds>>;

pub fn write_typenum_superscript<T: Integer>(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut v = T::to_i32();

    if v < 0 {
        f.write_str("⁻")?;
        v *= -1;
    }

    while v != 0 {
        f.write_str(match v % 10 {
            0 => "⁰",
            1 => "¹",
            2 => "²",
            3 => "³",
            4 => "⁴",
            5 => "⁵",
            6 => "⁶",
            7 => "⁷",
            8 => "⁸",
            9 => "⁹",
            _ => unreachable!()
        })?;

        v = v / 10;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;

    #[test]
    fn unit_conversions() {
        let value: Millimeters<u32> = 10u32.into_units();
        let converted: Meters<f64> = value.into_units();

        assert_eq!(value.value as f64 / 1000.0, converted.value);
    }

    #[test]
    fn print_mm() {
        let value: Millimeters<u32> = 10u32.into_units();

        let mut output = String::new();
        write!(output, "{}", value);
        assert_eq!(output, "10 × 10⁻³m")
    }

    #[test]
    fn print_ms2() {
        let value: MetersPerSecond2<i32> = (-5i32).into_units();

        let mut output = String::new();
        write!(output, "{}", value);
        assert_eq!(output, "-5 m·s⁻²")
    }
}