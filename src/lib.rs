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
impl <AScaling: Integer + Add<BScaling>, ASeconds: Integer + Add<BSeconds>, AMeters: Integer + Add<BMeters>, AGrams: Integer + Add<BGrams>, AAmperes: Integer + Add<BAmperes>, AKelvin: Integer + Add<BKelvin>, AMoles: Integer + Add<BMoles>, ACandelas: Integer + Add<BCandelas>, AByte: Integer + Add<BByte>, ARadians: Integer + Add<BRadians>, ASteradians: Integer + Add<BSteradians>, ACelsius: Integer + Add<BCelsius>, AMinutes: Integer + Add<BMinutes>, AHours: Integer + Add<BHours>, ADays: Integer + Add<BDays>, AAstronomicalUnits: Integer + Add<BAstronomicalUnits>, ADegrees: Integer + Add<BDegrees>, AArcminutes: Integer + Add<BArcminutes>, AArcseconds: Integer + Add<BArcseconds>, AAres: Integer + Add<BAres>, ALiters: Integer + Add<BLiters>, ADaltons: Integer + Add<BDaltons>, AElectronvolts: Integer + Add<BElectronvolts>, ANepers: Integer + Add<BNepers>, ABels: Integer + Add<BBels>, AAtmospheres: Integer + Add<BAtmospheres>, ABars: Integer + Add<BBars>, AParsec: Integer + Add<BParsec>, AMillimetersOfMercury: Integer + Add<BMillimetersOfMercury>, BScaling: Integer,BSeconds: Integer,BMeters: Integer,BGrams: Integer,BAmperes: Integer,BKelvin: Integer,BMoles: Integer,BCandelas: Integer,BByte: Integer,BRadians: Integer,BSteradians: Integer,BCelsius: Integer,BMinutes: Integer,BHours: Integer,BDays: Integer,BAstronomicalUnits: Integer,BDegrees: Integer,BArcminutes: Integer,BArcseconds: Integer,BAres: Integer,BLiters: Integer,BDaltons: Integer,BElectronvolts: Integer,BNepers: Integer,BBels: Integer,BAtmospheres: Integer,BBars: Integer,BParsec: Integer,BMillimetersOfMercury: Integer> Mul<DimensionStruct<BScaling, BSeconds, BMeters, BGrams, BAmperes, BKelvin, BMoles, BCandelas, BByte, BRadians, BSteradians, BCelsius, BMinutes, BHours, BDays, BAstronomicalUnits, BDegrees, BArcminutes, BArcseconds, BAres, BLiters, BDaltons, BElectronvolts, BNepers, BBels, BAtmospheres, BBars, BParsec, BMillimetersOfMercury>> for DimensionStruct<AScaling, ASeconds, AMeters, AGrams, AAmperes, AKelvin, AMoles, ACandelas, AByte, ARadians, ASteradians, ACelsius, AMinutes, AHours, ADays, AAstronomicalUnits, ADegrees, AArcminutes, AArcseconds, AAres, ALiters, ADaltons, AElectronvolts, ANepers, ABels, AAtmospheres, ABars, AParsec, AMillimetersOfMercury>
where <AScaling as Add<BScaling>>::Output: Integer, <ASeconds as Add<BSeconds>>::Output: Integer, <AMeters as Add<BMeters>>::Output: Integer, <AGrams as Add<BGrams>>::Output: Integer, <AAmperes as Add<BAmperes>>::Output: Integer, <AKelvin as Add<BKelvin>>::Output: Integer, <AMoles as Add<BMoles>>::Output: Integer, <ACandelas as Add<BCandelas>>::Output: Integer, <AByte as Add<BByte>>::Output: Integer, <ARadians as Add<BRadians>>::Output: Integer, <ASteradians as Add<BSteradians>>::Output: Integer, <ACelsius as Add<BCelsius>>::Output: Integer, <AMinutes as Add<BMinutes>>::Output: Integer, <AHours as Add<BHours>>::Output: Integer, <ADays as Add<BDays>>::Output: Integer, <AAstronomicalUnits as Add<BAstronomicalUnits>>::Output: Integer, <ADegrees as Add<BDegrees>>::Output: Integer, <AArcminutes as Add<BArcminutes>>::Output: Integer, <AArcseconds as Add<BArcseconds>>::Output: Integer, <AAres as Add<BAres>>::Output: Integer, <ALiters as Add<BLiters>>::Output: Integer, <ADaltons as Add<BDaltons>>::Output: Integer, <AElectronvolts as Add<BElectronvolts>>::Output: Integer, <ANepers as Add<BNepers>>::Output: Integer, <ABels as Add<BBels>>::Output: Integer, <AAtmospheres as Add<BAtmospheres>>::Output: Integer, <ABars as Add<BBars>>::Output: Integer, <AParsec as Add<BParsec>>::Output: Integer, <AMillimetersOfMercury as Add<BMillimetersOfMercury>>::Output: Integer {
    type Output = DimensionStruct<<AScaling as Add<BScaling>>::Output,<ASeconds as Add<BSeconds>>::Output,<AMeters as Add<BMeters>>::Output,<AGrams as Add<BGrams>>::Output,<AAmperes as Add<BAmperes>>::Output,<AKelvin as Add<BKelvin>>::Output,<AMoles as Add<BMoles>>::Output,<ACandelas as Add<BCandelas>>::Output,<AByte as Add<BByte>>::Output,<ARadians as Add<BRadians>>::Output,<ASteradians as Add<BSteradians>>::Output,<ACelsius as Add<BCelsius>>::Output,<AMinutes as Add<BMinutes>>::Output,<AHours as Add<BHours>>::Output,<ADays as Add<BDays>>::Output,<AAstronomicalUnits as Add<BAstronomicalUnits>>::Output,<ADegrees as Add<BDegrees>>::Output,<AArcminutes as Add<BArcminutes>>::Output,<AArcseconds as Add<BArcseconds>>::Output,<AAres as Add<BAres>>::Output,<ALiters as Add<BLiters>>::Output,<ADaltons as Add<BDaltons>>::Output,<AElectronvolts as Add<BElectronvolts>>::Output,<ANepers as Add<BNepers>>::Output,<ABels as Add<BBels>>::Output,<AAtmospheres as Add<BAtmospheres>>::Output,<ABars as Add<BBars>>::Output,<AParsec as Add<BParsec>>::Output,<AMillimetersOfMercury as Add<BMillimetersOfMercury>>::Output>;

    fn mul(self, rhs: DimensionStruct<BScaling, BSeconds, BMeters, BGrams, BAmperes, BKelvin, BMoles, BCandelas, BByte, BRadians, BSteradians, BCelsius, BMinutes, BHours, BDays, BAstronomicalUnits, BDegrees, BArcminutes, BArcseconds, BAres, BLiters, BDaltons, BElectronvolts, BNepers, BBels, BAtmospheres, BBars, BParsec, BMillimetersOfMercury>) -> Self::Output {
        DimensionStruct::new()
    }
}
impl <AScaling: Integer + Sub<BScaling>, ASeconds: Integer + Sub<BSeconds>, AMeters: Integer + Sub<BMeters>, AGrams: Integer + Sub<BGrams>, AAmperes: Integer + Sub<BAmperes>, AKelvin: Integer + Sub<BKelvin>, AMoles: Integer + Sub<BMoles>, ACandelas: Integer + Sub<BCandelas>, AByte: Integer + Sub<BByte>, ARadians: Integer + Sub<BRadians>, ASteradians: Integer + Sub<BSteradians>, ACelsius: Integer + Sub<BCelsius>, AMinutes: Integer + Sub<BMinutes>, AHours: Integer + Sub<BHours>, ADays: Integer + Sub<BDays>, AAstronomicalUnits: Integer + Sub<BAstronomicalUnits>, ADegrees: Integer + Sub<BDegrees>, AArcminutes: Integer + Sub<BArcminutes>, AArcseconds: Integer + Sub<BArcseconds>, AAres: Integer + Sub<BAres>, ALiters: Integer + Sub<BLiters>, ADaltons: Integer + Sub<BDaltons>, AElectronvolts: Integer + Sub<BElectronvolts>, ANepers: Integer + Sub<BNepers>, ABels: Integer + Sub<BBels>, AAtmospheres: Integer + Sub<BAtmospheres>, ABars: Integer + Sub<BBars>, AParsec: Integer + Sub<BParsec>, AMillimetersOfMercury: Integer + Sub<BMillimetersOfMercury>, BScaling: Integer,BSeconds: Integer,BMeters: Integer,BGrams: Integer,BAmperes: Integer,BKelvin: Integer,BMoles: Integer,BCandelas: Integer,BByte: Integer,BRadians: Integer,BSteradians: Integer,BCelsius: Integer,BMinutes: Integer,BHours: Integer,BDays: Integer,BAstronomicalUnits: Integer,BDegrees: Integer,BArcminutes: Integer,BArcseconds: Integer,BAres: Integer,BLiters: Integer,BDaltons: Integer,BElectronvolts: Integer,BNepers: Integer,BBels: Integer,BAtmospheres: Integer,BBars: Integer,BParsec: Integer,BMillimetersOfMercury: Integer> Div<DimensionStruct<BScaling, BSeconds, BMeters, BGrams, BAmperes, BKelvin, BMoles, BCandelas, BByte, BRadians, BSteradians, BCelsius, BMinutes, BHours, BDays, BAstronomicalUnits, BDegrees, BArcminutes, BArcseconds, BAres, BLiters, BDaltons, BElectronvolts, BNepers, BBels, BAtmospheres, BBars, BParsec, BMillimetersOfMercury>> for DimensionStruct<AScaling, ASeconds, AMeters, AGrams, AAmperes, AKelvin, AMoles, ACandelas, AByte, ARadians, ASteradians, ACelsius, AMinutes, AHours, ADays, AAstronomicalUnits, ADegrees, AArcminutes, AArcseconds, AAres, ALiters, ADaltons, AElectronvolts, ANepers, ABels, AAtmospheres, ABars, AParsec, AMillimetersOfMercury>
where <AScaling as Sub<BScaling>>::Output: Integer, <ASeconds as Sub<BSeconds>>::Output: Integer, <AMeters as Sub<BMeters>>::Output: Integer, <AGrams as Sub<BGrams>>::Output: Integer, <AAmperes as Sub<BAmperes>>::Output: Integer, <AKelvin as Sub<BKelvin>>::Output: Integer, <AMoles as Sub<BMoles>>::Output: Integer, <ACandelas as Sub<BCandelas>>::Output: Integer, <AByte as Sub<BByte>>::Output: Integer, <ARadians as Sub<BRadians>>::Output: Integer, <ASteradians as Sub<BSteradians>>::Output: Integer, <ACelsius as Sub<BCelsius>>::Output: Integer, <AMinutes as Sub<BMinutes>>::Output: Integer, <AHours as Sub<BHours>>::Output: Integer, <ADays as Sub<BDays>>::Output: Integer, <AAstronomicalUnits as Sub<BAstronomicalUnits>>::Output: Integer, <ADegrees as Sub<BDegrees>>::Output: Integer, <AArcminutes as Sub<BArcminutes>>::Output: Integer, <AArcseconds as Sub<BArcseconds>>::Output: Integer, <AAres as Sub<BAres>>::Output: Integer, <ALiters as Sub<BLiters>>::Output: Integer, <ADaltons as Sub<BDaltons>>::Output: Integer, <AElectronvolts as Sub<BElectronvolts>>::Output: Integer, <ANepers as Sub<BNepers>>::Output: Integer, <ABels as Sub<BBels>>::Output: Integer, <AAtmospheres as Sub<BAtmospheres>>::Output: Integer, <ABars as Sub<BBars>>::Output: Integer, <AParsec as Sub<BParsec>>::Output: Integer, <AMillimetersOfMercury as Sub<BMillimetersOfMercury>>::Output: Integer {
    type Output = DimensionStruct<<AScaling as Sub<BScaling>>::Output,<ASeconds as Sub<BSeconds>>::Output,<AMeters as Sub<BMeters>>::Output,<AGrams as Sub<BGrams>>::Output,<AAmperes as Sub<BAmperes>>::Output,<AKelvin as Sub<BKelvin>>::Output,<AMoles as Sub<BMoles>>::Output,<ACandelas as Sub<BCandelas>>::Output,<AByte as Sub<BByte>>::Output,<ARadians as Sub<BRadians>>::Output,<ASteradians as Sub<BSteradians>>::Output,<ACelsius as Sub<BCelsius>>::Output,<AMinutes as Sub<BMinutes>>::Output,<AHours as Sub<BHours>>::Output,<ADays as Sub<BDays>>::Output,<AAstronomicalUnits as Sub<BAstronomicalUnits>>::Output,<ADegrees as Sub<BDegrees>>::Output,<AArcminutes as Sub<BArcminutes>>::Output,<AArcseconds as Sub<BArcseconds>>::Output,<AAres as Sub<BAres>>::Output,<ALiters as Sub<BLiters>>::Output,<ADaltons as Sub<BDaltons>>::Output,<AElectronvolts as Sub<BElectronvolts>>::Output,<ANepers as Sub<BNepers>>::Output,<ABels as Sub<BBels>>::Output,<AAtmospheres as Sub<BAtmospheres>>::Output,<ABars as Sub<BBars>>::Output,<AParsec as Sub<BParsec>>::Output,<AMillimetersOfMercury as Sub<BMillimetersOfMercury>>::Output>;

    fn div(self, rhs: DimensionStruct<BScaling, BSeconds, BMeters, BGrams, BAmperes, BKelvin, BMoles, BCandelas, BByte, BRadians, BSteradians, BCelsius, BMinutes, BHours, BDays, BAstronomicalUnits, BDegrees, BArcminutes, BArcseconds, BAres, BLiters, BDaltons, BElectronvolts, BNepers, BBels, BAtmospheres, BBars, BParsec, BMillimetersOfMercury>) -> Self::Output {
        DimensionStruct::new()
    }
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
pub mod amps;
pub use amps::*;
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