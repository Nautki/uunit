#![no_std]
#![allow(unused)]

pub(crate) use core::marker::PhantomData;
pub(crate) use core::ops::*;
pub(crate) use typenum::*;

#[repr(transparent)]
#[derive(Clone, Copy, serde::Serialize, serde::Deserialize)]
#[derive(zerocopy::KnownLayout, zerocopy::Immutable, zerocopy::FromBytes, zerocopy::IntoBytes)]
#[serde(transparent)]
pub struct Quantity<T, D: Dimension + ?Sized> {
    pub value: T,
    pub dim: PhantomData<D>,
}

impl <T: core::fmt::Debug, D: Dimension + ?Sized> core::fmt::Debug for Quantity<T, D> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)?;
        // TODO: write unit string (e.g. so this method writes "32.123 Pascals")
        Ok(())
    }
}

impl<T, D: Dimension + ?Sized> Quantity<T, D> {
    pub fn new(value: T) -> Self {
        Quantity { value, dim: PhantomData }
    }
}
/// Re-interprets the unit WITHOUT conversion.
pub trait WithUnits {
    type Output<D: Dimension>;

    /// Re-interprets the units WITHOUT conversion.
    fn with_units<D: Dimension>(self) -> Self::Output<D>;
}

impl <T, I: Dimension + ?Sized> WithUnits for Quantity<T, I> {
    type Output<U: Dimension> = Quantity<T, U>;

    fn with_units<D: Dimension>(self) -> Quantity<T, D> {
        Quantity::new(self.value)
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