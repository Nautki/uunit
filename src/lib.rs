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
impl <D: Dimension> IntoUnits<Quantity<i8, D>> for i8 {    
    fn into_units(&self) -> Quantity<i8, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<i16, D>> for i16 {    
    fn into_units(&self) -> Quantity<i16, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<i32, D>> for i32 {    
    fn into_units(&self) -> Quantity<i32, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<i64, D>> for i64 {    
    fn into_units(&self) -> Quantity<i64, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<i128, D>> for i128 {    
    fn into_units(&self) -> Quantity<i128, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<isize, D>> for isize {    
    fn into_units(&self) -> Quantity<isize, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<u8, D>> for u8 {    
    fn into_units(&self) -> Quantity<u8, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<u16, D>> for u16 {    
    fn into_units(&self) -> Quantity<u16, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<u32, D>> for u32 {    
    fn into_units(&self) -> Quantity<u32, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<u64, D>> for u64 {    
    fn into_units(&self) -> Quantity<u64, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<u128, D>> for u128 {    
    fn into_units(&self) -> Quantity<u128, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<usize, D>> for usize {    
    fn into_units(&self) -> Quantity<usize, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<f32, D>> for f32 {    
    fn into_units(&self) -> Quantity<f32, D> {
        Quantity::new(*self)
    }
}
impl <D: Dimension> IntoUnits<Quantity<f64, D>> for f64 {    
    fn into_units(&self) -> Quantity<f64, D> {
        Quantity::new(*self)
    }
}
#[derive(Clone, Copy, Default)]
pub struct DimensionStruct<Scaling: Integer, Meters: Integer, Seconds: Integer, Grams: Integer, Amperes: Integer, Kelvin: Integer, Moles: Integer, Candelas: Integer, Byte: Integer, Radians: Integer, Steradians: Integer, Celsius: Integer, Minutes: Integer, Hours: Integer, Days: Integer, AstronomicalUnits: Integer, Degrees: Integer, Arcminutes: Integer, Arcseconds: Integer, Ares: Integer, Liters: Integer, Daltons: Integer, Electronvolts: Integer, Nepers: Integer, Bels: Integer, Atmospheres: Integer, Bars: Integer, Parsec: Integer, MillimetersOfMercury: Integer, Gs: Integer> {
    scaling: PhantomData<Scaling>,
    meters: PhantomData<Meters>,
    seconds: PhantomData<Seconds>,
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
    millimeters_of_mercury: PhantomData<MillimetersOfMercury>,
    gs: PhantomData<Gs>
}
impl <Scaling: Integer, Meters: Integer, Seconds: Integer, Grams: Integer, Amperes: Integer, Kelvin: Integer, Moles: Integer, Candelas: Integer, Byte: Integer, Radians: Integer, Steradians: Integer, Celsius: Integer, Minutes: Integer, Hours: Integer, Days: Integer, AstronomicalUnits: Integer, Degrees: Integer, Arcminutes: Integer, Arcseconds: Integer, Ares: Integer, Liters: Integer, Daltons: Integer, Electronvolts: Integer, Nepers: Integer, Bels: Integer, Atmospheres: Integer, Bars: Integer, Parsec: Integer, MillimetersOfMercury: Integer, Gs: Integer> DimensionStruct<Scaling, Meters, Seconds, Grams, Amperes, Kelvin, Moles, Candelas, Byte, Radians, Steradians, Celsius, Minutes, Hours, Days, AstronomicalUnits, Degrees, Arcminutes, Arcseconds, Ares, Liters, Daltons, Electronvolts, Nepers, Bels, Atmospheres, Bars, Parsec, MillimetersOfMercury, Gs> {
    pub fn new() -> Self {
        Self {
            scaling: PhantomData,
            meters: PhantomData,
            seconds: PhantomData,
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
            millimeters_of_mercury: PhantomData,
            gs: PhantomData
        }
    }
}
pub trait Dimension: Default + core::fmt::Debug + core::fmt::Display {
    type Scaling: Integer;
    type Meters: Integer;
    type Seconds: Integer;
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
    type Gs: Integer;
}
impl <Scaling: Integer, Meters: Integer, Seconds: Integer, Grams: Integer, Amperes: Integer, Kelvin: Integer, Moles: Integer, Candelas: Integer, Byte: Integer, Radians: Integer, Steradians: Integer, Celsius: Integer, Minutes: Integer, Hours: Integer, Days: Integer, AstronomicalUnits: Integer, Degrees: Integer, Arcminutes: Integer, Arcseconds: Integer, Ares: Integer, Liters: Integer, Daltons: Integer, Electronvolts: Integer, Nepers: Integer, Bels: Integer, Atmospheres: Integer, Bars: Integer, Parsec: Integer, MillimetersOfMercury: Integer, Gs: Integer> Dimension for DimensionStruct<Scaling, Meters, Seconds, Grams, Amperes, Kelvin, Moles, Candelas, Byte, Radians, Steradians, Celsius, Minutes, Hours, Days, AstronomicalUnits, Degrees, Arcminutes, Arcseconds, Ares, Liters, Daltons, Electronvolts, Nepers, Bels, Atmospheres, Bars, Parsec, MillimetersOfMercury, Gs> {
    type Scaling = Scaling;
    type Meters = Meters;
    type Seconds = Seconds;
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
    type Gs = Gs;
}
impl <Scaling: Integer, Meters: Integer, Seconds: Integer, Grams: Integer, Amperes: Integer, Kelvin: Integer, Moles: Integer, Candelas: Integer, Byte: Integer, Radians: Integer, Steradians: Integer, Celsius: Integer, Minutes: Integer, Hours: Integer, Days: Integer, AstronomicalUnits: Integer, Degrees: Integer, Arcminutes: Integer, Arcseconds: Integer, Ares: Integer, Liters: Integer, Daltons: Integer, Electronvolts: Integer, Nepers: Integer, Bels: Integer, Atmospheres: Integer, Bars: Integer, Parsec: Integer, MillimetersOfMercury: Integer, Gs: Integer> core::fmt::Display for DimensionStruct<Scaling, Meters, Seconds, Grams, Amperes, Kelvin, Moles, Candelas, Byte, Radians, Steradians, Celsius, Minutes, Hours, Days, AstronomicalUnits, Degrees, Arcminutes, Arcseconds, Ares, Liters, Daltons, Electronvolts, Nepers, Bels, Atmospheres, Bars, Parsec, MillimetersOfMercury, Gs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let scaling = Scaling::to_i32();
        if scaling != 0 {
            f.write_str("× 10");
            if scaling != 1 {
                write_typenum_superscript::<Scaling>(f)?;
            }
        }

        let mut is_first = true;

        let val = Meters::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("m")?;
            if val != 1 {
                write_typenum_superscript::<Meters>(f)?;
            }
            is_first = false;
        }
        
        let val = Seconds::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("s")?;
            if val != 1 {
                write_typenum_superscript::<Seconds>(f)?;
            }
            is_first = false;
        }
        
        let val = Grams::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("g")?;
            if val != 1 {
                write_typenum_superscript::<Grams>(f)?;
            }
            is_first = false;
        }
        
        let val = Amperes::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("A")?;
            if val != 1 {
                write_typenum_superscript::<Amperes>(f)?;
            }
            is_first = false;
        }
        
        let val = Kelvin::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("K")?;
            if val != 1 {
                write_typenum_superscript::<Kelvin>(f)?;
            }
            is_first = false;
        }
        
        let val = Moles::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("mol")?;
            if val != 1 {
                write_typenum_superscript::<Moles>(f)?;
            }
            is_first = false;
        }
        
        let val = Candelas::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("ca")?;
            if val != 1 {
                write_typenum_superscript::<Candelas>(f)?;
            }
            is_first = false;
        }
        
        let val = Byte::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("b")?;
            if val != 1 {
                write_typenum_superscript::<Byte>(f)?;
            }
            is_first = false;
        }
        
        let val = Radians::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("radians")?;
            if val != 1 {
                write_typenum_superscript::<Radians>(f)?;
            }
            is_first = false;
        }
        
        let val = Steradians::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("steradians")?;
            if val != 1 {
                write_typenum_superscript::<Steradians>(f)?;
            }
            is_first = false;
        }
        
        let val = Celsius::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("celsius")?;
            if val != 1 {
                write_typenum_superscript::<Celsius>(f)?;
            }
            is_first = false;
        }
        
        let val = Minutes::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("minutes")?;
            if val != 1 {
                write_typenum_superscript::<Minutes>(f)?;
            }
            is_first = false;
        }
        
        let val = Hours::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("hours")?;
            if val != 1 {
                write_typenum_superscript::<Hours>(f)?;
            }
            is_first = false;
        }
        
        let val = Days::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("days")?;
            if val != 1 {
                write_typenum_superscript::<Days>(f)?;
            }
            is_first = false;
        }
        
        let val = AstronomicalUnits::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("astronomicalUnits")?;
            if val != 1 {
                write_typenum_superscript::<AstronomicalUnits>(f)?;
            }
            is_first = false;
        }
        
        let val = Degrees::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("degrees")?;
            if val != 1 {
                write_typenum_superscript::<Degrees>(f)?;
            }
            is_first = false;
        }
        
        let val = Arcminutes::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("arcminutes")?;
            if val != 1 {
                write_typenum_superscript::<Arcminutes>(f)?;
            }
            is_first = false;
        }
        
        let val = Arcseconds::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("arcseconds")?;
            if val != 1 {
                write_typenum_superscript::<Arcseconds>(f)?;
            }
            is_first = false;
        }
        
        let val = Ares::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("ares")?;
            if val != 1 {
                write_typenum_superscript::<Ares>(f)?;
            }
            is_first = false;
        }
        
        let val = Liters::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("liters")?;
            if val != 1 {
                write_typenum_superscript::<Liters>(f)?;
            }
            is_first = false;
        }
        
        let val = Daltons::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("daltons")?;
            if val != 1 {
                write_typenum_superscript::<Daltons>(f)?;
            }
            is_first = false;
        }
        
        let val = Electronvolts::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("electronvolts")?;
            if val != 1 {
                write_typenum_superscript::<Electronvolts>(f)?;
            }
            is_first = false;
        }
        
        let val = Nepers::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("nepers")?;
            if val != 1 {
                write_typenum_superscript::<Nepers>(f)?;
            }
            is_first = false;
        }
        
        let val = Bels::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("bels")?;
            if val != 1 {
                write_typenum_superscript::<Bels>(f)?;
            }
            is_first = false;
        }
        
        let val = Atmospheres::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("atmospheres")?;
            if val != 1 {
                write_typenum_superscript::<Atmospheres>(f)?;
            }
            is_first = false;
        }
        
        let val = Bars::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("bars")?;
            if val != 1 {
                write_typenum_superscript::<Bars>(f)?;
            }
            is_first = false;
        }
        
        let val = Parsec::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("parsec")?;
            if val != 1 {
                write_typenum_superscript::<Parsec>(f)?;
            }
            is_first = false;
        }
        
        let val = MillimetersOfMercury::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("millimetersOfMercury")?;
            if val != 1 {
                write_typenum_superscript::<MillimetersOfMercury>(f)?;
            }
            is_first = false;
        }
        
        let val = Gs::to_i32();
        if val != 0 {
            if !is_first {
                f.write_str("·")?;
            }
            f.write_str("Gs")?;
            if val != 1 {
                write_typenum_superscript::<Gs>(f)?;
            }
            is_first = false;
        }
        Ok(())
    }
}
impl <Scaling: Integer, Meters: Integer, Seconds: Integer, Grams: Integer, Amperes: Integer, Kelvin: Integer, Moles: Integer, Candelas: Integer, Byte: Integer, Radians: Integer, Steradians: Integer, Celsius: Integer, Minutes: Integer, Hours: Integer, Days: Integer, AstronomicalUnits: Integer, Degrees: Integer, Arcminutes: Integer, Arcseconds: Integer, Ares: Integer, Liters: Integer, Daltons: Integer, Electronvolts: Integer, Nepers: Integer, Bels: Integer, Atmospheres: Integer, Bars: Integer, Parsec: Integer, MillimetersOfMercury: Integer, Gs: Integer> core::fmt::Debug for DimensionStruct<Scaling, Meters, Seconds, Grams, Amperes, Kelvin, Moles, Candelas, Byte, Radians, Steradians, Celsius, Minutes, Hours, Days, AstronomicalUnits, Degrees, Arcminutes, Arcseconds, Ares, Liters, Daltons, Electronvolts, Nepers, Bels, Atmospheres, Bars, Parsec, MillimetersOfMercury, Gs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(self, f)
    }
}
impl <
    Meters,Seconds,Grams,Amperes,Kelvin,Moles,Candelas,Byte,Radians,Steradians,Celsius,Minutes,Hours,Days,AstronomicalUnits,Degrees,Arcminutes,Arcseconds,Ares,Liters,Daltons,Electronvolts,Nepers,Bels,Atmospheres,Bars,Parsec,MillimetersOfMercury,Gs,
    A,
    AD,
    B,
    BD
> FromUnits<Quantity<A, AD>> for Quantity<B, BD> where
    AD: Dimension<Meters = Meters,Seconds = Seconds,Grams = Grams,Amperes = Amperes,Kelvin = Kelvin,Moles = Moles,Candelas = Candelas,Byte = Byte,Radians = Radians,Steradians = Steradians,Celsius = Celsius,Minutes = Minutes,Hours = Hours,Days = Days,AstronomicalUnits = AstronomicalUnits,Degrees = Degrees,Arcminutes = Arcminutes,Arcseconds = Arcseconds,Ares = Ares,Liters = Liters,Daltons = Daltons,Electronvolts = Electronvolts,Nepers = Nepers,Bels = Bels,Atmospheres = Atmospheres,Bars = Bars,Parsec = Parsec,MillimetersOfMercury = MillimetersOfMercury,Gs = Gs> + ?Sized,
    A: Clone,
    B: From<A> + From<u8> + Mul<B, Output = B> + Div<B, Output = B> + Clone,
    BD: Dimension<Meters = Meters,Seconds = Seconds,Grams = Grams,Amperes = Amperes,Kelvin = Kelvin,Moles = Moles,Candelas = Candelas,Byte = Byte,Radians = Radians,Steradians = Steradians,Celsius = Celsius,Minutes = Minutes,Hours = Hours,Days = Days,AstronomicalUnits = AstronomicalUnits,Degrees = Degrees,Arcminutes = Arcminutes,Arcseconds = Arcseconds,Ares = Ares,Liters = Liters,Daltons = Daltons,Electronvolts = Electronvolts,Nepers = Nepers,Bels = Bels,Atmospheres = Atmospheres,Bars = Bars,Parsec = Parsec,MillimetersOfMercury = MillimetersOfMercury,Gs = Gs> + ?Sized,
    BD::Scaling: Sub::<AD::Scaling>,
    <BD::Scaling as Sub::<AD::Scaling>>::Output: Integer
{
    fn from_units(quantity: &Quantity<A, AD>) -> Self {
        let mut value: B = quantity.value.clone().into();
        let pow = -<<BD::Scaling as Sub::<AD::Scaling>>::Output as Integer>::ISIZE;

        let ten: B = 10.into();
        if pow < 0 {
            for _ in 0..-pow {
                value = value / ten.clone();
            }
        } else {
            for _ in 0..pow {
                value = value * ten.clone();
            }
        }

        Quantity::new(value)
    }
}
impl <AScaling: Integer + Add<BScaling>, AMeters: Integer + Add<BMeters>, ASeconds: Integer + Add<BSeconds>, AGrams: Integer + Add<BGrams>, AAmperes: Integer + Add<BAmperes>, AKelvin: Integer + Add<BKelvin>, AMoles: Integer + Add<BMoles>, ACandelas: Integer + Add<BCandelas>, AByte: Integer + Add<BByte>, ARadians: Integer + Add<BRadians>, ASteradians: Integer + Add<BSteradians>, ACelsius: Integer + Add<BCelsius>, AMinutes: Integer + Add<BMinutes>, AHours: Integer + Add<BHours>, ADays: Integer + Add<BDays>, AAstronomicalUnits: Integer + Add<BAstronomicalUnits>, ADegrees: Integer + Add<BDegrees>, AArcminutes: Integer + Add<BArcminutes>, AArcseconds: Integer + Add<BArcseconds>, AAres: Integer + Add<BAres>, ALiters: Integer + Add<BLiters>, ADaltons: Integer + Add<BDaltons>, AElectronvolts: Integer + Add<BElectronvolts>, ANepers: Integer + Add<BNepers>, ABels: Integer + Add<BBels>, AAtmospheres: Integer + Add<BAtmospheres>, ABars: Integer + Add<BBars>, AParsec: Integer + Add<BParsec>, AMillimetersOfMercury: Integer + Add<BMillimetersOfMercury>, AGs: Integer + Add<BGs>, BScaling: Integer,BMeters: Integer,BSeconds: Integer,BGrams: Integer,BAmperes: Integer,BKelvin: Integer,BMoles: Integer,BCandelas: Integer,BByte: Integer,BRadians: Integer,BSteradians: Integer,BCelsius: Integer,BMinutes: Integer,BHours: Integer,BDays: Integer,BAstronomicalUnits: Integer,BDegrees: Integer,BArcminutes: Integer,BArcseconds: Integer,BAres: Integer,BLiters: Integer,BDaltons: Integer,BElectronvolts: Integer,BNepers: Integer,BBels: Integer,BAtmospheres: Integer,BBars: Integer,BParsec: Integer,BMillimetersOfMercury: Integer,BGs: Integer> Mul<DimensionStruct<BScaling, BMeters, BSeconds, BGrams, BAmperes, BKelvin, BMoles, BCandelas, BByte, BRadians, BSteradians, BCelsius, BMinutes, BHours, BDays, BAstronomicalUnits, BDegrees, BArcminutes, BArcseconds, BAres, BLiters, BDaltons, BElectronvolts, BNepers, BBels, BAtmospheres, BBars, BParsec, BMillimetersOfMercury, BGs>> for DimensionStruct<AScaling, AMeters, ASeconds, AGrams, AAmperes, AKelvin, AMoles, ACandelas, AByte, ARadians, ASteradians, ACelsius, AMinutes, AHours, ADays, AAstronomicalUnits, ADegrees, AArcminutes, AArcseconds, AAres, ALiters, ADaltons, AElectronvolts, ANepers, ABels, AAtmospheres, ABars, AParsec, AMillimetersOfMercury, AGs>
where <AScaling as Add<BScaling>>::Output: Integer, <AMeters as Add<BMeters>>::Output: Integer, <ASeconds as Add<BSeconds>>::Output: Integer, <AGrams as Add<BGrams>>::Output: Integer, <AAmperes as Add<BAmperes>>::Output: Integer, <AKelvin as Add<BKelvin>>::Output: Integer, <AMoles as Add<BMoles>>::Output: Integer, <ACandelas as Add<BCandelas>>::Output: Integer, <AByte as Add<BByte>>::Output: Integer, <ARadians as Add<BRadians>>::Output: Integer, <ASteradians as Add<BSteradians>>::Output: Integer, <ACelsius as Add<BCelsius>>::Output: Integer, <AMinutes as Add<BMinutes>>::Output: Integer, <AHours as Add<BHours>>::Output: Integer, <ADays as Add<BDays>>::Output: Integer, <AAstronomicalUnits as Add<BAstronomicalUnits>>::Output: Integer, <ADegrees as Add<BDegrees>>::Output: Integer, <AArcminutes as Add<BArcminutes>>::Output: Integer, <AArcseconds as Add<BArcseconds>>::Output: Integer, <AAres as Add<BAres>>::Output: Integer, <ALiters as Add<BLiters>>::Output: Integer, <ADaltons as Add<BDaltons>>::Output: Integer, <AElectronvolts as Add<BElectronvolts>>::Output: Integer, <ANepers as Add<BNepers>>::Output: Integer, <ABels as Add<BBels>>::Output: Integer, <AAtmospheres as Add<BAtmospheres>>::Output: Integer, <ABars as Add<BBars>>::Output: Integer, <AParsec as Add<BParsec>>::Output: Integer, <AMillimetersOfMercury as Add<BMillimetersOfMercury>>::Output: Integer, <AGs as Add<BGs>>::Output: Integer {
    type Output = DimensionStruct<<AScaling as Add<BScaling>>::Output,<AMeters as Add<BMeters>>::Output,<ASeconds as Add<BSeconds>>::Output,<AGrams as Add<BGrams>>::Output,<AAmperes as Add<BAmperes>>::Output,<AKelvin as Add<BKelvin>>::Output,<AMoles as Add<BMoles>>::Output,<ACandelas as Add<BCandelas>>::Output,<AByte as Add<BByte>>::Output,<ARadians as Add<BRadians>>::Output,<ASteradians as Add<BSteradians>>::Output,<ACelsius as Add<BCelsius>>::Output,<AMinutes as Add<BMinutes>>::Output,<AHours as Add<BHours>>::Output,<ADays as Add<BDays>>::Output,<AAstronomicalUnits as Add<BAstronomicalUnits>>::Output,<ADegrees as Add<BDegrees>>::Output,<AArcminutes as Add<BArcminutes>>::Output,<AArcseconds as Add<BArcseconds>>::Output,<AAres as Add<BAres>>::Output,<ALiters as Add<BLiters>>::Output,<ADaltons as Add<BDaltons>>::Output,<AElectronvolts as Add<BElectronvolts>>::Output,<ANepers as Add<BNepers>>::Output,<ABels as Add<BBels>>::Output,<AAtmospheres as Add<BAtmospheres>>::Output,<ABars as Add<BBars>>::Output,<AParsec as Add<BParsec>>::Output,<AMillimetersOfMercury as Add<BMillimetersOfMercury>>::Output,<AGs as Add<BGs>>::Output>;

    fn mul(self, rhs: DimensionStruct<BScaling, BMeters, BSeconds, BGrams, BAmperes, BKelvin, BMoles, BCandelas, BByte, BRadians, BSteradians, BCelsius, BMinutes, BHours, BDays, BAstronomicalUnits, BDegrees, BArcminutes, BArcseconds, BAres, BLiters, BDaltons, BElectronvolts, BNepers, BBels, BAtmospheres, BBars, BParsec, BMillimetersOfMercury, BGs>) -> Self::Output {
        DimensionStruct::new()
    }
}
impl <AScaling: Integer + Sub<BScaling>, AMeters: Integer + Sub<BMeters>, ASeconds: Integer + Sub<BSeconds>, AGrams: Integer + Sub<BGrams>, AAmperes: Integer + Sub<BAmperes>, AKelvin: Integer + Sub<BKelvin>, AMoles: Integer + Sub<BMoles>, ACandelas: Integer + Sub<BCandelas>, AByte: Integer + Sub<BByte>, ARadians: Integer + Sub<BRadians>, ASteradians: Integer + Sub<BSteradians>, ACelsius: Integer + Sub<BCelsius>, AMinutes: Integer + Sub<BMinutes>, AHours: Integer + Sub<BHours>, ADays: Integer + Sub<BDays>, AAstronomicalUnits: Integer + Sub<BAstronomicalUnits>, ADegrees: Integer + Sub<BDegrees>, AArcminutes: Integer + Sub<BArcminutes>, AArcseconds: Integer + Sub<BArcseconds>, AAres: Integer + Sub<BAres>, ALiters: Integer + Sub<BLiters>, ADaltons: Integer + Sub<BDaltons>, AElectronvolts: Integer + Sub<BElectronvolts>, ANepers: Integer + Sub<BNepers>, ABels: Integer + Sub<BBels>, AAtmospheres: Integer + Sub<BAtmospheres>, ABars: Integer + Sub<BBars>, AParsec: Integer + Sub<BParsec>, AMillimetersOfMercury: Integer + Sub<BMillimetersOfMercury>, AGs: Integer + Sub<BGs>, BScaling: Integer,BMeters: Integer,BSeconds: Integer,BGrams: Integer,BAmperes: Integer,BKelvin: Integer,BMoles: Integer,BCandelas: Integer,BByte: Integer,BRadians: Integer,BSteradians: Integer,BCelsius: Integer,BMinutes: Integer,BHours: Integer,BDays: Integer,BAstronomicalUnits: Integer,BDegrees: Integer,BArcminutes: Integer,BArcseconds: Integer,BAres: Integer,BLiters: Integer,BDaltons: Integer,BElectronvolts: Integer,BNepers: Integer,BBels: Integer,BAtmospheres: Integer,BBars: Integer,BParsec: Integer,BMillimetersOfMercury: Integer,BGs: Integer> Div<DimensionStruct<BScaling, BMeters, BSeconds, BGrams, BAmperes, BKelvin, BMoles, BCandelas, BByte, BRadians, BSteradians, BCelsius, BMinutes, BHours, BDays, BAstronomicalUnits, BDegrees, BArcminutes, BArcseconds, BAres, BLiters, BDaltons, BElectronvolts, BNepers, BBels, BAtmospheres, BBars, BParsec, BMillimetersOfMercury, BGs>> for DimensionStruct<AScaling, AMeters, ASeconds, AGrams, AAmperes, AKelvin, AMoles, ACandelas, AByte, ARadians, ASteradians, ACelsius, AMinutes, AHours, ADays, AAstronomicalUnits, ADegrees, AArcminutes, AArcseconds, AAres, ALiters, ADaltons, AElectronvolts, ANepers, ABels, AAtmospheres, ABars, AParsec, AMillimetersOfMercury, AGs>
where <AScaling as Sub<BScaling>>::Output: Integer, <AMeters as Sub<BMeters>>::Output: Integer, <ASeconds as Sub<BSeconds>>::Output: Integer, <AGrams as Sub<BGrams>>::Output: Integer, <AAmperes as Sub<BAmperes>>::Output: Integer, <AKelvin as Sub<BKelvin>>::Output: Integer, <AMoles as Sub<BMoles>>::Output: Integer, <ACandelas as Sub<BCandelas>>::Output: Integer, <AByte as Sub<BByte>>::Output: Integer, <ARadians as Sub<BRadians>>::Output: Integer, <ASteradians as Sub<BSteradians>>::Output: Integer, <ACelsius as Sub<BCelsius>>::Output: Integer, <AMinutes as Sub<BMinutes>>::Output: Integer, <AHours as Sub<BHours>>::Output: Integer, <ADays as Sub<BDays>>::Output: Integer, <AAstronomicalUnits as Sub<BAstronomicalUnits>>::Output: Integer, <ADegrees as Sub<BDegrees>>::Output: Integer, <AArcminutes as Sub<BArcminutes>>::Output: Integer, <AArcseconds as Sub<BArcseconds>>::Output: Integer, <AAres as Sub<BAres>>::Output: Integer, <ALiters as Sub<BLiters>>::Output: Integer, <ADaltons as Sub<BDaltons>>::Output: Integer, <AElectronvolts as Sub<BElectronvolts>>::Output: Integer, <ANepers as Sub<BNepers>>::Output: Integer, <ABels as Sub<BBels>>::Output: Integer, <AAtmospheres as Sub<BAtmospheres>>::Output: Integer, <ABars as Sub<BBars>>::Output: Integer, <AParsec as Sub<BParsec>>::Output: Integer, <AMillimetersOfMercury as Sub<BMillimetersOfMercury>>::Output: Integer, <AGs as Sub<BGs>>::Output: Integer {
    type Output = DimensionStruct<<AScaling as Sub<BScaling>>::Output,<AMeters as Sub<BMeters>>::Output,<ASeconds as Sub<BSeconds>>::Output,<AGrams as Sub<BGrams>>::Output,<AAmperes as Sub<BAmperes>>::Output,<AKelvin as Sub<BKelvin>>::Output,<AMoles as Sub<BMoles>>::Output,<ACandelas as Sub<BCandelas>>::Output,<AByte as Sub<BByte>>::Output,<ARadians as Sub<BRadians>>::Output,<ASteradians as Sub<BSteradians>>::Output,<ACelsius as Sub<BCelsius>>::Output,<AMinutes as Sub<BMinutes>>::Output,<AHours as Sub<BHours>>::Output,<ADays as Sub<BDays>>::Output,<AAstronomicalUnits as Sub<BAstronomicalUnits>>::Output,<ADegrees as Sub<BDegrees>>::Output,<AArcminutes as Sub<BArcminutes>>::Output,<AArcseconds as Sub<BArcseconds>>::Output,<AAres as Sub<BAres>>::Output,<ALiters as Sub<BLiters>>::Output,<ADaltons as Sub<BDaltons>>::Output,<AElectronvolts as Sub<BElectronvolts>>::Output,<ANepers as Sub<BNepers>>::Output,<ABels as Sub<BBels>>::Output,<AAtmospheres as Sub<BAtmospheres>>::Output,<ABars as Sub<BBars>>::Output,<AParsec as Sub<BParsec>>::Output,<AMillimetersOfMercury as Sub<BMillimetersOfMercury>>::Output,<AGs as Sub<BGs>>::Output>;

    fn div(self, rhs: DimensionStruct<BScaling, BMeters, BSeconds, BGrams, BAmperes, BKelvin, BMoles, BCandelas, BByte, BRadians, BSteradians, BCelsius, BMinutes, BHours, BDays, BAstronomicalUnits, BDegrees, BArcminutes, BArcseconds, BAres, BLiters, BDaltons, BElectronvolts, BNepers, BBels, BAtmospheres, BBars, BParsec, BMillimetersOfMercury, BGs>) -> Self::Output {
        DimensionStruct::new()
    }
}
pub mod meters;
pub use meters::*;
pub mod seconds;
pub use seconds::*;
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
pub mod gs;
pub use gs::*;
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