#![no_std]
#![allow(unused)]

pub(crate) use core::marker::PhantomData;
pub(crate) use core::ops::*;
pub(crate) use typenum::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Quantity<T, D: Dimension + ?Sized> {
    pub value: T,
    pub dim: PhantomData<D>,
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

impl<T: Mul<T, Output = T>, D: Dimension + Mul<D>> Mul<Quantity<T, D>> for Quantity<T, D>
where <D as Mul<D>>::Output: Dimension {
    type Output = Quantity<T, <D as Mul<D>>::Output>;
    fn mul(self, rhs: Quantity<T, D>) -> Self::Output {
        Quantity::new(self.value.mul(rhs.value))
    }
}

impl<T: Div<T, Output = T>, D: Dimension + Div<D>> Div<Quantity<T, D>> for Quantity<T, D>
where <D as Div<D>>::Output: Dimension {
    type Output = Quantity<T, <D as Div<D>>::Output>;
    fn div(self, rhs: Quantity<T, D>) -> Self::Output {
        Quantity::new(self.value.div(rhs.value))
    }
}

pub type Multiply<A, B> = <A as Mul<B>>::Output;
pub type Divide<N, D> = <N as Div<D>>::Output;
impl WithUnits for i8 {
    type Output<D: Dimension> = Quantity<i8, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for i16 {
    type Output<D: Dimension> = Quantity<i16, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for i32 {
    type Output<D: Dimension> = Quantity<i32, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for i64 {
    type Output<D: Dimension> = Quantity<i64, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for i128 {
    type Output<D: Dimension> = Quantity<i128, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for isize {
    type Output<D: Dimension> = Quantity<isize, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for u8 {
    type Output<D: Dimension> = Quantity<u8, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for u16 {
    type Output<D: Dimension> = Quantity<u16, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for u32 {
    type Output<D: Dimension> = Quantity<u32, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for u64 {
    type Output<D: Dimension> = Quantity<u64, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for u128 {
    type Output<D: Dimension> = Quantity<u128, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for usize {
    type Output<D: Dimension> = Quantity<usize, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for f32 {
    type Output<D: Dimension> = Quantity<f32, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
impl WithUnits for f64 {
    type Output<D: Dimension> = Quantity<f64, D>;
    
    fn with_units<D: Dimension>(self) -> Self::Output<D> {
        Quantity::new(self)
    }
}
#[derive(Clone, Copy)]
pub struct DimensionStruct<Scaling: Integer, Seconds: Integer, Meters: Integer, Grams: Integer, Amperes: Integer, Kelvin: Integer, Moles: Integer, Candelas: Integer, Byte: Integer, Radians: Integer, Steradians: Integer, Celsius: Integer, Minutes: Integer, Hours: Integer, Days: Integer, AstronomicalUnits: Integer, Degrees: Integer, Arcminutes: Integer, Arcseconds: Integer, Ares: Integer, Liters: Integer, Daltons: Integer, Electronvolts: Integer, Nepers: Integer, Bels: Integer, Atmospheres: Integer, Bars: Integer, Parsec: Integer, MillimetersOfMercury: Integer> {
    scaling: PhantomData<Scaling>,
    seconds: PhantomData<Seconds>,
    meters: PhantomData<Meters>,
    grams: PhantomData<Grams>,
    amperes: PhantomData<Amperes>,
    kelvin: PhantomData<Kelvin>,
    moles: PhantomData<Moles>,
    candelas: PhantomData<Candelas>,
    byte: PhantomData<Byte>,
    radians: PhantomData<Radians>,
    steradians: PhantomData<Steradians>,
    celsius: PhantomData<Celsius>,
    minutes: PhantomData<Minutes>,
    hours: PhantomData<Hours>,
    days: PhantomData<Days>,
    astronomical_units: PhantomData<AstronomicalUnits>,
    degrees: PhantomData<Degrees>,
    arcminutes: PhantomData<Arcminutes>,
    arcseconds: PhantomData<Arcseconds>,
    ares: PhantomData<Ares>,
    liters: PhantomData<Liters>,
    daltons: PhantomData<Daltons>,
    electronvolts: PhantomData<Electronvolts>,
    nepers: PhantomData<Nepers>,
    bels: PhantomData<Bels>,
    atmospheres: PhantomData<Atmospheres>,
    bars: PhantomData<Bars>,
    parsec: PhantomData<Parsec>,
    millimeters_of_mercury: PhantomData<MillimetersOfMercury>
}
impl <Scaling: Integer, Seconds: Integer, Meters: Integer, Grams: Integer, Amperes: Integer, Kelvin: Integer, Moles: Integer, Candelas: Integer, Byte: Integer, Radians: Integer, Steradians: Integer, Celsius: Integer, Minutes: Integer, Hours: Integer, Days: Integer, AstronomicalUnits: Integer, Degrees: Integer, Arcminutes: Integer, Arcseconds: Integer, Ares: Integer, Liters: Integer, Daltons: Integer, Electronvolts: Integer, Nepers: Integer, Bels: Integer, Atmospheres: Integer, Bars: Integer, Parsec: Integer, MillimetersOfMercury: Integer> DimensionStruct<Scaling, Seconds, Meters, Grams, Amperes, Kelvin, Moles, Candelas, Byte, Radians, Steradians, Celsius, Minutes, Hours, Days, AstronomicalUnits, Degrees, Arcminutes, Arcseconds, Ares, Liters, Daltons, Electronvolts, Nepers, Bels, Atmospheres, Bars, Parsec, MillimetersOfMercury> {
    pub fn new() -> Self {
        Self {
            scaling: PhantomData,
            seconds: PhantomData,
            meters: PhantomData,
            grams: PhantomData,
            amperes: PhantomData,
            kelvin: PhantomData,
            moles: PhantomData,
            candelas: PhantomData,
            byte: PhantomData,
            radians: PhantomData,
            steradians: PhantomData,
            celsius: PhantomData,
            minutes: PhantomData,
            hours: PhantomData,
            days: PhantomData,
            astronomical_units: PhantomData,
            degrees: PhantomData,
            arcminutes: PhantomData,
            arcseconds: PhantomData,
            ares: PhantomData,
            liters: PhantomData,
            daltons: PhantomData,
            electronvolts: PhantomData,
            nepers: PhantomData,
            bels: PhantomData,
            atmospheres: PhantomData,
            bars: PhantomData,
            parsec: PhantomData,
            millimeters_of_mercury: PhantomData
        }
    }
}
pub trait Dimension {
    type Scaling: Integer;
    type Seconds: Integer;
    type Meters: Integer;
    type Grams: Integer;
    type Amperes: Integer;
    type Kelvin: Integer;
    type Moles: Integer;
    type Candelas: Integer;
    type Byte: Integer;
    type Radians: Integer;
    type Steradians: Integer;
    type Celsius: Integer;
    type Minutes: Integer;
    type Hours: Integer;
    type Days: Integer;
    type AstronomicalUnits: Integer;
    type Degrees: Integer;
    type Arcminutes: Integer;
    type Arcseconds: Integer;
    type Ares: Integer;
    type Liters: Integer;
    type Daltons: Integer;
    type Electronvolts: Integer;
    type Nepers: Integer;
    type Bels: Integer;
    type Atmospheres: Integer;
    type Bars: Integer;
    type Parsec: Integer;
    type MillimetersOfMercury: Integer;
}
impl <Scaling: Integer, Seconds: Integer, Meters: Integer, Grams: Integer, Amperes: Integer, Kelvin: Integer, Moles: Integer, Candelas: Integer, Byte: Integer, Radians: Integer, Steradians: Integer, Celsius: Integer, Minutes: Integer, Hours: Integer, Days: Integer, AstronomicalUnits: Integer, Degrees: Integer, Arcminutes: Integer, Arcseconds: Integer, Ares: Integer, Liters: Integer, Daltons: Integer, Electronvolts: Integer, Nepers: Integer, Bels: Integer, Atmospheres: Integer, Bars: Integer, Parsec: Integer, MillimetersOfMercury: Integer> Dimension for DimensionStruct<Scaling, Seconds, Meters, Grams, Amperes, Kelvin, Moles, Candelas, Byte, Radians, Steradians, Celsius, Minutes, Hours, Days, AstronomicalUnits, Degrees, Arcminutes, Arcseconds, Ares, Liters, Daltons, Electronvolts, Nepers, Bels, Atmospheres, Bars, Parsec, MillimetersOfMercury> {
    type Scaling = Scaling;
    type Seconds = Seconds;
    type Meters = Meters;
    type Grams = Grams;
    type Amperes = Amperes;
    type Kelvin = Kelvin;
    type Moles = Moles;
    type Candelas = Candelas;
    type Byte = Byte;
    type Radians = Radians;
    type Steradians = Steradians;
    type Celsius = Celsius;
    type Minutes = Minutes;
    type Hours = Hours;
    type Days = Days;
    type AstronomicalUnits = AstronomicalUnits;
    type Degrees = Degrees;
    type Arcminutes = Arcminutes;
    type Arcseconds = Arcseconds;
    type Ares = Ares;
    type Liters = Liters;
    type Daltons = Daltons;
    type Electronvolts = Electronvolts;
    type Nepers = Nepers;
    type Bels = Bels;
    type Atmospheres = Atmospheres;
    type Bars = Bars;
    type Parsec = Parsec;
    type MillimetersOfMercury = MillimetersOfMercury;
}
pub mod seconds;
pub use seconds::*;
pub mod meters;
pub use meters::*;
pub mod grams;
pub use grams::*;
pub mod amperes;
pub use amperes::*;
pub mod kelvin;
pub use kelvin::*;
pub mod moles;
pub use moles::*;
pub mod candelas;
pub use candelas::*;
pub mod byte;
pub use byte::*;
pub mod radians;
pub use radians::*;
pub mod steradians;
pub use steradians::*;
pub mod celsius;
pub use celsius::*;
pub mod minutes;
pub use minutes::*;
pub mod hours;
pub use hours::*;
pub mod days;
pub use days::*;
pub mod astronomical_units;
pub use astronomical_units::*;
pub mod degrees;
pub use degrees::*;
pub mod arcminutes;
pub use arcminutes::*;
pub mod arcseconds;
pub use arcseconds::*;
pub mod ares;
pub use ares::*;
pub mod liters;
pub use liters::*;
pub mod daltons;
pub use daltons::*;
pub mod electronvolts;
pub use electronvolts::*;
pub mod nepers;
pub use nepers::*;
pub mod bels;
pub use bels::*;
pub mod atmospheres;
pub use atmospheres::*;
pub mod bars;
pub use bars::*;
pub mod parsec;
pub use parsec::*;
pub mod millimeters_of_mercury;
pub use millimeters_of_mercury::*;
pub mod micron;
pub use micron::*;
pub mod fermi;
pub use fermi::*;
pub mod metric_ton;
pub use metric_ton::*;
pub mod hertz;
pub use hertz::*;
pub mod newtons;
pub use newtons::*;
pub mod pascals;
pub use pascals::*;
pub mod joules;
pub use joules::*;
pub mod watts;
pub use watts::*;
pub mod coulombs;
pub use coulombs::*;
pub mod volts;
pub use volts::*;
pub mod farads;
pub use farads::*;
pub mod ohms;
pub use ohms::*;
pub mod siemens;
pub use siemens::*;
pub mod webers;
pub use webers::*;
pub mod teslas;
pub use teslas::*;
pub mod henries;
pub use henries::*;
pub mod lumens;
pub use lumens::*;
pub mod lux;
pub use lux::*;
pub mod becquerels;
pub use becquerels::*;
pub mod grays;
pub use grays::*;
pub mod sieverts;
pub use sieverts::*;
pub mod katals;
pub use katals::*;