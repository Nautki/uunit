#![no_std]
#![allow(unused)]

pub(crate) use core::marker::PhantomData;
pub(crate) use typenum::*;

#[derive(Debug, Clone, Copy)]
pub struct Quantity<T, D: Dimension> {
    pub value: T,
    pub dim: D,
}

impl<T, D: Dimension> Quantity<T, D> {
    pub fn new(value: T, dim: D) -> Self {
        Quantity { value, dim }
    }
}
#[derive(Clone, Copy)]
pub struct DimensionStruct<TScaling: Integer, TSeconds: Integer, TMeters: Integer, TGrams: Integer, TAmperes: Integer, TKelvin: Integer, TMoles: Integer, TCandelas: Integer, TByte: Integer, TRadians: Integer, TSteradians: Integer, TCelsius: Integer, TMinutes: Integer, THours: Integer, TDays: Integer, TAstronomicalUnits: Integer, TDegrees: Integer, TArcminutes: Integer, TArcseconds: Integer, TAres: Integer, TLiters: Integer, TDaltons: Integer, TElectronvolts: Integer, TNepers: Integer, TBels: Integer, TAtmospheres: Integer, TBars: Integer, TParsec: Integer, TMillimetersOfMercury: Integer> {
    scaling: PhantomData<TScaling>,
    seconds: PhantomData<TSeconds>,
    meters: PhantomData<TMeters>,
    grams: PhantomData<TGrams>,
    amperes: PhantomData<TAmperes>,
    kelvin: PhantomData<TKelvin>,
    moles: PhantomData<TMoles>,
    candelas: PhantomData<TCandelas>,
    byte: PhantomData<TByte>,
    radians: PhantomData<TRadians>,
    steradians: PhantomData<TSteradians>,
    celsius: PhantomData<TCelsius>,
    minutes: PhantomData<TMinutes>,
    hours: PhantomData<THours>,
    days: PhantomData<TDays>,
    astronomical_units: PhantomData<TAstronomicalUnits>,
    degrees: PhantomData<TDegrees>,
    arcminutes: PhantomData<TArcminutes>,
    arcseconds: PhantomData<TArcseconds>,
    ares: PhantomData<TAres>,
    liters: PhantomData<TLiters>,
    daltons: PhantomData<TDaltons>,
    electronvolts: PhantomData<TElectronvolts>,
    nepers: PhantomData<TNepers>,
    bels: PhantomData<TBels>,
    atmospheres: PhantomData<TAtmospheres>,
    bars: PhantomData<TBars>,
    parsec: PhantomData<TParsec>,
    millimeters_of_mercury: PhantomData<TMillimetersOfMercury>
}
impl <TScaling: Integer, TSeconds: Integer, TMeters: Integer, TGrams: Integer, TAmperes: Integer, TKelvin: Integer, TMoles: Integer, TCandelas: Integer, TByte: Integer, TRadians: Integer, TSteradians: Integer, TCelsius: Integer, TMinutes: Integer, THours: Integer, TDays: Integer, TAstronomicalUnits: Integer, TDegrees: Integer, TArcminutes: Integer, TArcseconds: Integer, TAres: Integer, TLiters: Integer, TDaltons: Integer, TElectronvolts: Integer, TNepers: Integer, TBels: Integer, TAtmospheres: Integer, TBars: Integer, TParsec: Integer, TMillimetersOfMercury: Integer> DimensionStruct<TScaling, TSeconds, TMeters, TGrams, TAmperes, TKelvin, TMoles, TCandelas, TByte, TRadians, TSteradians, TCelsius, TMinutes, THours, TDays, TAstronomicalUnits, TDegrees, TArcminutes, TArcseconds, TAres, TLiters, TDaltons, TElectronvolts, TNepers, TBels, TAtmospheres, TBars, TParsec, TMillimetersOfMercury> {
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
impl <TScaling: Integer, TSeconds: Integer, TMeters: Integer, TGrams: Integer, TAmperes: Integer, TKelvin: Integer, TMoles: Integer, TCandelas: Integer, TByte: Integer, TRadians: Integer, TSteradians: Integer, TCelsius: Integer, TMinutes: Integer, THours: Integer, TDays: Integer, TAstronomicalUnits: Integer, TDegrees: Integer, TArcminutes: Integer, TArcseconds: Integer, TAres: Integer, TLiters: Integer, TDaltons: Integer, TElectronvolts: Integer, TNepers: Integer, TBels: Integer, TAtmospheres: Integer, TBars: Integer, TParsec: Integer, TMillimetersOfMercury: Integer> Dimension for DimensionStruct<TScaling, TSeconds, TMeters, TGrams, TAmperes, TKelvin, TMoles, TCandelas, TByte, TRadians, TSteradians, TCelsius, TMinutes, THours, TDays, TAstronomicalUnits, TDegrees, TArcminutes, TArcseconds, TAres, TLiters, TDaltons, TElectronvolts, TNepers, TBels, TAtmospheres, TBars, TParsec, TMillimetersOfMercury> {
    type Scaling = TScaling;
    type Seconds = TSeconds;
    type Meters = TMeters;
    type Grams = TGrams;
    type Amperes = TAmperes;
    type Kelvin = TKelvin;
    type Moles = TMoles;
    type Candelas = TCandelas;
    type Byte = TByte;
    type Radians = TRadians;
    type Steradians = TSteradians;
    type Celsius = TCelsius;
    type Minutes = TMinutes;
    type Hours = THours;
    type Days = TDays;
    type AstronomicalUnits = TAstronomicalUnits;
    type Degrees = TDegrees;
    type Arcminutes = TArcminutes;
    type Arcseconds = TArcseconds;
    type Ares = TAres;
    type Liters = TLiters;
    type Daltons = TDaltons;
    type Electronvolts = TElectronvolts;
    type Nepers = TNepers;
    type Bels = TBels;
    type Atmospheres = TAtmospheres;
    type Bars = TBars;
    type Parsec = TParsec;
    type MillimetersOfMercury = TMillimetersOfMercury;
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
pub trait DimExt: Copy {


    fn as_seconds(self) -> Seconds<Self> {
        Quantity::new(self, UnitSeconds::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaseconds(self) -> Quettaseconds<Self> {
        Quantity::new(self, UnitQuettaseconds::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaseconds(self) -> Ronnaseconds<Self> {
        Quantity::new(self, UnitRonnaseconds::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaseconds(self) -> Yottaseconds<Self> {
        Quantity::new(self, UnitYottaseconds::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaseconds(self) -> Zettaseconds<Self> {
        Quantity::new(self, UnitZettaseconds::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaseconds(self) -> Exaseconds<Self> {
        Quantity::new(self, UnitExaseconds::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaseconds(self) -> Petaseconds<Self> {
        Quantity::new(self, UnitPetaseconds::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraseconds(self) -> Teraseconds<Self> {
        Quantity::new(self, UnitTeraseconds::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaseconds(self) -> Gigaseconds<Self> {
        Quantity::new(self, UnitGigaseconds::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaseconds(self) -> Megaseconds<Self> {
        Quantity::new(self, UnitMegaseconds::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloseconds(self) -> Kiloseconds<Self> {
        Quantity::new(self, UnitKiloseconds::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoseconds(self) -> Hectoseconds<Self> {
        Quantity::new(self, UnitHectoseconds::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaseconds(self) -> Decaseconds<Self> {
        Quantity::new(self, UnitDecaseconds::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciseconds(self) -> Deciseconds<Self> {
        Quantity::new(self, UnitDeciseconds::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiseconds(self) -> Centiseconds<Self> {
        Quantity::new(self, UnitCentiseconds::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliseconds(self) -> Milliseconds<Self> {
        Quantity::new(self, UnitMilliseconds::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microseconds(self) -> Microseconds<Self> {
        Quantity::new(self, UnitMicroseconds::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoseconds(self) -> Nanoseconds<Self> {
        Quantity::new(self, UnitNanoseconds::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoseconds(self) -> Picoseconds<Self> {
        Quantity::new(self, UnitPicoseconds::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoseconds(self) -> Femtoseconds<Self> {
        Quantity::new(self, UnitFemtoseconds::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoseconds(self) -> Attoseconds<Self> {
        Quantity::new(self, UnitAttoseconds::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoseconds(self) -> Zeptoseconds<Self> {
        Quantity::new(self, UnitZeptoseconds::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoseconds(self) -> Yoctoseconds<Self> {
        Quantity::new(self, UnitYoctoseconds::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoseconds(self) -> Rontoseconds<Self> {
        Quantity::new(self, UnitRontoseconds::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoseconds(self) -> Quectoseconds<Self> {
        Quantity::new(self, UnitQuectoseconds::new())
    }


    fn as_meters(self) -> Meters<Self> {
        Quantity::new(self, UnitMeters::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettameters(self) -> Quettameters<Self> {
        Quantity::new(self, UnitQuettameters::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnameters(self) -> Ronnameters<Self> {
        Quantity::new(self, UnitRonnameters::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottameters(self) -> Yottameters<Self> {
        Quantity::new(self, UnitYottameters::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettameters(self) -> Zettameters<Self> {
        Quantity::new(self, UnitZettameters::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exameters(self) -> Exameters<Self> {
        Quantity::new(self, UnitExameters::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petameters(self) -> Petameters<Self> {
        Quantity::new(self, UnitPetameters::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terameters(self) -> Terameters<Self> {
        Quantity::new(self, UnitTerameters::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigameters(self) -> Gigameters<Self> {
        Quantity::new(self, UnitGigameters::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megameters(self) -> Megameters<Self> {
        Quantity::new(self, UnitMegameters::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilometers(self) -> Kilometers<Self> {
        Quantity::new(self, UnitKilometers::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectometers(self) -> Hectometers<Self> {
        Quantity::new(self, UnitHectometers::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decameters(self) -> Decameters<Self> {
        Quantity::new(self, UnitDecameters::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decimeters(self) -> Decimeters<Self> {
        Quantity::new(self, UnitDecimeters::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centimeters(self) -> Centimeters<Self> {
        Quantity::new(self, UnitCentimeters::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millimeters(self) -> Millimeters<Self> {
        Quantity::new(self, UnitMillimeters::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_micrometers(self) -> Micrometers<Self> {
        Quantity::new(self, UnitMicrometers::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanometers(self) -> Nanometers<Self> {
        Quantity::new(self, UnitNanometers::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picometers(self) -> Picometers<Self> {
        Quantity::new(self, UnitPicometers::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtometers(self) -> Femtometers<Self> {
        Quantity::new(self, UnitFemtometers::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attometers(self) -> Attometers<Self> {
        Quantity::new(self, UnitAttometers::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptometers(self) -> Zeptometers<Self> {
        Quantity::new(self, UnitZeptometers::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctometers(self) -> Yoctometers<Self> {
        Quantity::new(self, UnitYoctometers::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontometers(self) -> Rontometers<Self> {
        Quantity::new(self, UnitRontometers::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectometers(self) -> Quectometers<Self> {
        Quantity::new(self, UnitQuectometers::new())
    }


    fn as_grams(self) -> Grams<Self> {
        Quantity::new(self, UnitGrams::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettagrams(self) -> Quettagrams<Self> {
        Quantity::new(self, UnitQuettagrams::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnagrams(self) -> Ronnagrams<Self> {
        Quantity::new(self, UnitRonnagrams::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottagrams(self) -> Yottagrams<Self> {
        Quantity::new(self, UnitYottagrams::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettagrams(self) -> Zettagrams<Self> {
        Quantity::new(self, UnitZettagrams::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exagrams(self) -> Exagrams<Self> {
        Quantity::new(self, UnitExagrams::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petagrams(self) -> Petagrams<Self> {
        Quantity::new(self, UnitPetagrams::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teragrams(self) -> Teragrams<Self> {
        Quantity::new(self, UnitTeragrams::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigagrams(self) -> Gigagrams<Self> {
        Quantity::new(self, UnitGigagrams::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megagrams(self) -> Megagrams<Self> {
        Quantity::new(self, UnitMegagrams::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilograms(self) -> Kilograms<Self> {
        Quantity::new(self, UnitKilograms::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectograms(self) -> Hectograms<Self> {
        Quantity::new(self, UnitHectograms::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decagrams(self) -> Decagrams<Self> {
        Quantity::new(self, UnitDecagrams::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decigrams(self) -> Decigrams<Self> {
        Quantity::new(self, UnitDecigrams::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centigrams(self) -> Centigrams<Self> {
        Quantity::new(self, UnitCentigrams::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milligrams(self) -> Milligrams<Self> {
        Quantity::new(self, UnitMilligrams::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_micrograms(self) -> Micrograms<Self> {
        Quantity::new(self, UnitMicrograms::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanograms(self) -> Nanograms<Self> {
        Quantity::new(self, UnitNanograms::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picograms(self) -> Picograms<Self> {
        Quantity::new(self, UnitPicograms::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtograms(self) -> Femtograms<Self> {
        Quantity::new(self, UnitFemtograms::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attograms(self) -> Attograms<Self> {
        Quantity::new(self, UnitAttograms::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptograms(self) -> Zeptograms<Self> {
        Quantity::new(self, UnitZeptograms::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctograms(self) -> Yoctograms<Self> {
        Quantity::new(self, UnitYoctograms::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontograms(self) -> Rontograms<Self> {
        Quantity::new(self, UnitRontograms::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectograms(self) -> Quectograms<Self> {
        Quantity::new(self, UnitQuectograms::new())
    }


    fn as_amperes(self) -> Amperes<Self> {
        Quantity::new(self, UnitAmperes::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaamperes(self) -> Quettaamperes<Self> {
        Quantity::new(self, UnitQuettaamperes::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaamperes(self) -> Ronnaamperes<Self> {
        Quantity::new(self, UnitRonnaamperes::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaamperes(self) -> Yottaamperes<Self> {
        Quantity::new(self, UnitYottaamperes::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaamperes(self) -> Zettaamperes<Self> {
        Quantity::new(self, UnitZettaamperes::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaamperes(self) -> Exaamperes<Self> {
        Quantity::new(self, UnitExaamperes::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaamperes(self) -> Petaamperes<Self> {
        Quantity::new(self, UnitPetaamperes::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraamperes(self) -> Teraamperes<Self> {
        Quantity::new(self, UnitTeraamperes::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaamperes(self) -> Gigaamperes<Self> {
        Quantity::new(self, UnitGigaamperes::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaamperes(self) -> Megaamperes<Self> {
        Quantity::new(self, UnitMegaamperes::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloamperes(self) -> Kiloamperes<Self> {
        Quantity::new(self, UnitKiloamperes::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoamperes(self) -> Hectoamperes<Self> {
        Quantity::new(self, UnitHectoamperes::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaamperes(self) -> Decaamperes<Self> {
        Quantity::new(self, UnitDecaamperes::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciamperes(self) -> Deciamperes<Self> {
        Quantity::new(self, UnitDeciamperes::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiamperes(self) -> Centiamperes<Self> {
        Quantity::new(self, UnitCentiamperes::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliamperes(self) -> Milliamperes<Self> {
        Quantity::new(self, UnitMilliamperes::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microamperes(self) -> Microamperes<Self> {
        Quantity::new(self, UnitMicroamperes::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoamperes(self) -> Nanoamperes<Self> {
        Quantity::new(self, UnitNanoamperes::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoamperes(self) -> Picoamperes<Self> {
        Quantity::new(self, UnitPicoamperes::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoamperes(self) -> Femtoamperes<Self> {
        Quantity::new(self, UnitFemtoamperes::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoamperes(self) -> Attoamperes<Self> {
        Quantity::new(self, UnitAttoamperes::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoamperes(self) -> Zeptoamperes<Self> {
        Quantity::new(self, UnitZeptoamperes::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoamperes(self) -> Yoctoamperes<Self> {
        Quantity::new(self, UnitYoctoamperes::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoamperes(self) -> Rontoamperes<Self> {
        Quantity::new(self, UnitRontoamperes::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoamperes(self) -> Quectoamperes<Self> {
        Quantity::new(self, UnitQuectoamperes::new())
    }


    fn as_kelvin(self) -> Kelvin<Self> {
        Quantity::new(self, UnitKelvin::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettakelvin(self) -> Quettakelvin<Self> {
        Quantity::new(self, UnitQuettakelvin::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnakelvin(self) -> Ronnakelvin<Self> {
        Quantity::new(self, UnitRonnakelvin::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottakelvin(self) -> Yottakelvin<Self> {
        Quantity::new(self, UnitYottakelvin::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettakelvin(self) -> Zettakelvin<Self> {
        Quantity::new(self, UnitZettakelvin::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exakelvin(self) -> Exakelvin<Self> {
        Quantity::new(self, UnitExakelvin::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petakelvin(self) -> Petakelvin<Self> {
        Quantity::new(self, UnitPetakelvin::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terakelvin(self) -> Terakelvin<Self> {
        Quantity::new(self, UnitTerakelvin::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigakelvin(self) -> Gigakelvin<Self> {
        Quantity::new(self, UnitGigakelvin::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megakelvin(self) -> Megakelvin<Self> {
        Quantity::new(self, UnitMegakelvin::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilokelvin(self) -> Kilokelvin<Self> {
        Quantity::new(self, UnitKilokelvin::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectokelvin(self) -> Hectokelvin<Self> {
        Quantity::new(self, UnitHectokelvin::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decakelvin(self) -> Decakelvin<Self> {
        Quantity::new(self, UnitDecakelvin::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decikelvin(self) -> Decikelvin<Self> {
        Quantity::new(self, UnitDecikelvin::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centikelvin(self) -> Centikelvin<Self> {
        Quantity::new(self, UnitCentikelvin::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millikelvin(self) -> Millikelvin<Self> {
        Quantity::new(self, UnitMillikelvin::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microkelvin(self) -> Microkelvin<Self> {
        Quantity::new(self, UnitMicrokelvin::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanokelvin(self) -> Nanokelvin<Self> {
        Quantity::new(self, UnitNanokelvin::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picokelvin(self) -> Picokelvin<Self> {
        Quantity::new(self, UnitPicokelvin::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtokelvin(self) -> Femtokelvin<Self> {
        Quantity::new(self, UnitFemtokelvin::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attokelvin(self) -> Attokelvin<Self> {
        Quantity::new(self, UnitAttokelvin::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptokelvin(self) -> Zeptokelvin<Self> {
        Quantity::new(self, UnitZeptokelvin::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctokelvin(self) -> Yoctokelvin<Self> {
        Quantity::new(self, UnitYoctokelvin::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontokelvin(self) -> Rontokelvin<Self> {
        Quantity::new(self, UnitRontokelvin::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectokelvin(self) -> Quectokelvin<Self> {
        Quantity::new(self, UnitQuectokelvin::new())
    }


    fn as_moles(self) -> Moles<Self> {
        Quantity::new(self, UnitMoles::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettamoles(self) -> Quettamoles<Self> {
        Quantity::new(self, UnitQuettamoles::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnamoles(self) -> Ronnamoles<Self> {
        Quantity::new(self, UnitRonnamoles::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottamoles(self) -> Yottamoles<Self> {
        Quantity::new(self, UnitYottamoles::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettamoles(self) -> Zettamoles<Self> {
        Quantity::new(self, UnitZettamoles::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_examoles(self) -> Examoles<Self> {
        Quantity::new(self, UnitExamoles::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petamoles(self) -> Petamoles<Self> {
        Quantity::new(self, UnitPetamoles::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teramoles(self) -> Teramoles<Self> {
        Quantity::new(self, UnitTeramoles::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigamoles(self) -> Gigamoles<Self> {
        Quantity::new(self, UnitGigamoles::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megamoles(self) -> Megamoles<Self> {
        Quantity::new(self, UnitMegamoles::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilomoles(self) -> Kilomoles<Self> {
        Quantity::new(self, UnitKilomoles::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectomoles(self) -> Hectomoles<Self> {
        Quantity::new(self, UnitHectomoles::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decamoles(self) -> Decamoles<Self> {
        Quantity::new(self, UnitDecamoles::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decimoles(self) -> Decimoles<Self> {
        Quantity::new(self, UnitDecimoles::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centimoles(self) -> Centimoles<Self> {
        Quantity::new(self, UnitCentimoles::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millimoles(self) -> Millimoles<Self> {
        Quantity::new(self, UnitMillimoles::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_micromoles(self) -> Micromoles<Self> {
        Quantity::new(self, UnitMicromoles::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanomoles(self) -> Nanomoles<Self> {
        Quantity::new(self, UnitNanomoles::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picomoles(self) -> Picomoles<Self> {
        Quantity::new(self, UnitPicomoles::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtomoles(self) -> Femtomoles<Self> {
        Quantity::new(self, UnitFemtomoles::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attomoles(self) -> Attomoles<Self> {
        Quantity::new(self, UnitAttomoles::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptomoles(self) -> Zeptomoles<Self> {
        Quantity::new(self, UnitZeptomoles::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctomoles(self) -> Yoctomoles<Self> {
        Quantity::new(self, UnitYoctomoles::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontomoles(self) -> Rontomoles<Self> {
        Quantity::new(self, UnitRontomoles::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectomoles(self) -> Quectomoles<Self> {
        Quantity::new(self, UnitQuectomoles::new())
    }


    fn as_candelas(self) -> Candelas<Self> {
        Quantity::new(self, UnitCandelas::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettacandelas(self) -> Quettacandelas<Self> {
        Quantity::new(self, UnitQuettacandelas::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnacandelas(self) -> Ronnacandelas<Self> {
        Quantity::new(self, UnitRonnacandelas::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottacandelas(self) -> Yottacandelas<Self> {
        Quantity::new(self, UnitYottacandelas::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettacandelas(self) -> Zettacandelas<Self> {
        Quantity::new(self, UnitZettacandelas::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exacandelas(self) -> Exacandelas<Self> {
        Quantity::new(self, UnitExacandelas::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petacandelas(self) -> Petacandelas<Self> {
        Quantity::new(self, UnitPetacandelas::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teracandelas(self) -> Teracandelas<Self> {
        Quantity::new(self, UnitTeracandelas::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigacandelas(self) -> Gigacandelas<Self> {
        Quantity::new(self, UnitGigacandelas::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megacandelas(self) -> Megacandelas<Self> {
        Quantity::new(self, UnitMegacandelas::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilocandelas(self) -> Kilocandelas<Self> {
        Quantity::new(self, UnitKilocandelas::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectocandelas(self) -> Hectocandelas<Self> {
        Quantity::new(self, UnitHectocandelas::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decacandelas(self) -> Decacandelas<Self> {
        Quantity::new(self, UnitDecacandelas::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decicandelas(self) -> Decicandelas<Self> {
        Quantity::new(self, UnitDecicandelas::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centicandelas(self) -> Centicandelas<Self> {
        Quantity::new(self, UnitCenticandelas::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millicandelas(self) -> Millicandelas<Self> {
        Quantity::new(self, UnitMillicandelas::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microcandelas(self) -> Microcandelas<Self> {
        Quantity::new(self, UnitMicrocandelas::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanocandelas(self) -> Nanocandelas<Self> {
        Quantity::new(self, UnitNanocandelas::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picocandelas(self) -> Picocandelas<Self> {
        Quantity::new(self, UnitPicocandelas::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtocandelas(self) -> Femtocandelas<Self> {
        Quantity::new(self, UnitFemtocandelas::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attocandelas(self) -> Attocandelas<Self> {
        Quantity::new(self, UnitAttocandelas::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptocandelas(self) -> Zeptocandelas<Self> {
        Quantity::new(self, UnitZeptocandelas::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctocandelas(self) -> Yoctocandelas<Self> {
        Quantity::new(self, UnitYoctocandelas::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontocandelas(self) -> Rontocandelas<Self> {
        Quantity::new(self, UnitRontocandelas::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectocandelas(self) -> Quectocandelas<Self> {
        Quantity::new(self, UnitQuectocandelas::new())
    }


    fn as_byte(self) -> Byte<Self> {
        Quantity::new(self, UnitByte::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettabyte(self) -> Quettabyte<Self> {
        Quantity::new(self, UnitQuettabyte::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnabyte(self) -> Ronnabyte<Self> {
        Quantity::new(self, UnitRonnabyte::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottabyte(self) -> Yottabyte<Self> {
        Quantity::new(self, UnitYottabyte::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettabyte(self) -> Zettabyte<Self> {
        Quantity::new(self, UnitZettabyte::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exabyte(self) -> Exabyte<Self> {
        Quantity::new(self, UnitExabyte::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petabyte(self) -> Petabyte<Self> {
        Quantity::new(self, UnitPetabyte::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terabyte(self) -> Terabyte<Self> {
        Quantity::new(self, UnitTerabyte::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigabyte(self) -> Gigabyte<Self> {
        Quantity::new(self, UnitGigabyte::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megabyte(self) -> Megabyte<Self> {
        Quantity::new(self, UnitMegabyte::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilobyte(self) -> Kilobyte<Self> {
        Quantity::new(self, UnitKilobyte::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectobyte(self) -> Hectobyte<Self> {
        Quantity::new(self, UnitHectobyte::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decabyte(self) -> Decabyte<Self> {
        Quantity::new(self, UnitDecabyte::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decibyte(self) -> Decibyte<Self> {
        Quantity::new(self, UnitDecibyte::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centibyte(self) -> Centibyte<Self> {
        Quantity::new(self, UnitCentibyte::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millibyte(self) -> Millibyte<Self> {
        Quantity::new(self, UnitMillibyte::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microbyte(self) -> Microbyte<Self> {
        Quantity::new(self, UnitMicrobyte::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanobyte(self) -> Nanobyte<Self> {
        Quantity::new(self, UnitNanobyte::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picobyte(self) -> Picobyte<Self> {
        Quantity::new(self, UnitPicobyte::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtobyte(self) -> Femtobyte<Self> {
        Quantity::new(self, UnitFemtobyte::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attobyte(self) -> Attobyte<Self> {
        Quantity::new(self, UnitAttobyte::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptobyte(self) -> Zeptobyte<Self> {
        Quantity::new(self, UnitZeptobyte::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctobyte(self) -> Yoctobyte<Self> {
        Quantity::new(self, UnitYoctobyte::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontobyte(self) -> Rontobyte<Self> {
        Quantity::new(self, UnitRontobyte::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectobyte(self) -> Quectobyte<Self> {
        Quantity::new(self, UnitQuectobyte::new())
    }


    fn as_radians(self) -> Radians<Self> {
        Quantity::new(self, UnitRadians::new())
    }


    fn as_steradians(self) -> Steradians<Self> {
        Quantity::new(self, UnitSteradians::new())
    }


    fn as_celsius(self) -> Celsius<Self> {
        Quantity::new(self, UnitCelsius::new())
    }


    fn as_minutes(self) -> Minutes<Self> {
        Quantity::new(self, UnitMinutes::new())
    }


    fn as_hours(self) -> Hours<Self> {
        Quantity::new(self, UnitHours::new())
    }


    fn as_days(self) -> Days<Self> {
        Quantity::new(self, UnitDays::new())
    }


    fn as_astronomical_units(self) -> AstronomicalUnits<Self> {
        Quantity::new(self, UnitAstronomicalUnits::new())
    }


    fn as_degrees(self) -> Degrees<Self> {
        Quantity::new(self, UnitDegrees::new())
    }


    fn as_arcminutes(self) -> Arcminutes<Self> {
        Quantity::new(self, UnitArcminutes::new())
    }


    fn as_arcseconds(self) -> Arcseconds<Self> {
        Quantity::new(self, UnitArcseconds::new())
    }


    fn as_ares(self) -> Ares<Self> {
        Quantity::new(self, UnitAres::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaares(self) -> Quettaares<Self> {
        Quantity::new(self, UnitQuettaares::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaares(self) -> Ronnaares<Self> {
        Quantity::new(self, UnitRonnaares::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaares(self) -> Yottaares<Self> {
        Quantity::new(self, UnitYottaares::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaares(self) -> Zettaares<Self> {
        Quantity::new(self, UnitZettaares::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaares(self) -> Exaares<Self> {
        Quantity::new(self, UnitExaares::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaares(self) -> Petaares<Self> {
        Quantity::new(self, UnitPetaares::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraares(self) -> Teraares<Self> {
        Quantity::new(self, UnitTeraares::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaares(self) -> Gigaares<Self> {
        Quantity::new(self, UnitGigaares::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaares(self) -> Megaares<Self> {
        Quantity::new(self, UnitMegaares::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloares(self) -> Kiloares<Self> {
        Quantity::new(self, UnitKiloares::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoares(self) -> Hectoares<Self> {
        Quantity::new(self, UnitHectoares::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaares(self) -> Decaares<Self> {
        Quantity::new(self, UnitDecaares::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciares(self) -> Deciares<Self> {
        Quantity::new(self, UnitDeciares::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiares(self) -> Centiares<Self> {
        Quantity::new(self, UnitCentiares::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliares(self) -> Milliares<Self> {
        Quantity::new(self, UnitMilliares::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microares(self) -> Microares<Self> {
        Quantity::new(self, UnitMicroares::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoares(self) -> Nanoares<Self> {
        Quantity::new(self, UnitNanoares::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoares(self) -> Picoares<Self> {
        Quantity::new(self, UnitPicoares::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoares(self) -> Femtoares<Self> {
        Quantity::new(self, UnitFemtoares::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoares(self) -> Attoares<Self> {
        Quantity::new(self, UnitAttoares::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoares(self) -> Zeptoares<Self> {
        Quantity::new(self, UnitZeptoares::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoares(self) -> Yoctoares<Self> {
        Quantity::new(self, UnitYoctoares::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoares(self) -> Rontoares<Self> {
        Quantity::new(self, UnitRontoares::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoares(self) -> Quectoares<Self> {
        Quantity::new(self, UnitQuectoares::new())
    }


    fn as_liters(self) -> Liters<Self> {
        Quantity::new(self, UnitLiters::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaliters(self) -> Quettaliters<Self> {
        Quantity::new(self, UnitQuettaliters::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaliters(self) -> Ronnaliters<Self> {
        Quantity::new(self, UnitRonnaliters::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaliters(self) -> Yottaliters<Self> {
        Quantity::new(self, UnitYottaliters::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaliters(self) -> Zettaliters<Self> {
        Quantity::new(self, UnitZettaliters::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaliters(self) -> Exaliters<Self> {
        Quantity::new(self, UnitExaliters::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaliters(self) -> Petaliters<Self> {
        Quantity::new(self, UnitPetaliters::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraliters(self) -> Teraliters<Self> {
        Quantity::new(self, UnitTeraliters::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaliters(self) -> Gigaliters<Self> {
        Quantity::new(self, UnitGigaliters::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaliters(self) -> Megaliters<Self> {
        Quantity::new(self, UnitMegaliters::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloliters(self) -> Kiloliters<Self> {
        Quantity::new(self, UnitKiloliters::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoliters(self) -> Hectoliters<Self> {
        Quantity::new(self, UnitHectoliters::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaliters(self) -> Decaliters<Self> {
        Quantity::new(self, UnitDecaliters::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciliters(self) -> Deciliters<Self> {
        Quantity::new(self, UnitDeciliters::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiliters(self) -> Centiliters<Self> {
        Quantity::new(self, UnitCentiliters::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliliters(self) -> Milliliters<Self> {
        Quantity::new(self, UnitMilliliters::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microliters(self) -> Microliters<Self> {
        Quantity::new(self, UnitMicroliters::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoliters(self) -> Nanoliters<Self> {
        Quantity::new(self, UnitNanoliters::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoliters(self) -> Picoliters<Self> {
        Quantity::new(self, UnitPicoliters::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoliters(self) -> Femtoliters<Self> {
        Quantity::new(self, UnitFemtoliters::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoliters(self) -> Attoliters<Self> {
        Quantity::new(self, UnitAttoliters::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoliters(self) -> Zeptoliters<Self> {
        Quantity::new(self, UnitZeptoliters::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoliters(self) -> Yoctoliters<Self> {
        Quantity::new(self, UnitYoctoliters::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoliters(self) -> Rontoliters<Self> {
        Quantity::new(self, UnitRontoliters::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoliters(self) -> Quectoliters<Self> {
        Quantity::new(self, UnitQuectoliters::new())
    }


    fn as_daltons(self) -> Daltons<Self> {
        Quantity::new(self, UnitDaltons::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettadaltons(self) -> Quettadaltons<Self> {
        Quantity::new(self, UnitQuettadaltons::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnadaltons(self) -> Ronnadaltons<Self> {
        Quantity::new(self, UnitRonnadaltons::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottadaltons(self) -> Yottadaltons<Self> {
        Quantity::new(self, UnitYottadaltons::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettadaltons(self) -> Zettadaltons<Self> {
        Quantity::new(self, UnitZettadaltons::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exadaltons(self) -> Exadaltons<Self> {
        Quantity::new(self, UnitExadaltons::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petadaltons(self) -> Petadaltons<Self> {
        Quantity::new(self, UnitPetadaltons::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teradaltons(self) -> Teradaltons<Self> {
        Quantity::new(self, UnitTeradaltons::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigadaltons(self) -> Gigadaltons<Self> {
        Quantity::new(self, UnitGigadaltons::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megadaltons(self) -> Megadaltons<Self> {
        Quantity::new(self, UnitMegadaltons::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilodaltons(self) -> Kilodaltons<Self> {
        Quantity::new(self, UnitKilodaltons::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectodaltons(self) -> Hectodaltons<Self> {
        Quantity::new(self, UnitHectodaltons::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decadaltons(self) -> Decadaltons<Self> {
        Quantity::new(self, UnitDecadaltons::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decidaltons(self) -> Decidaltons<Self> {
        Quantity::new(self, UnitDecidaltons::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centidaltons(self) -> Centidaltons<Self> {
        Quantity::new(self, UnitCentidaltons::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millidaltons(self) -> Millidaltons<Self> {
        Quantity::new(self, UnitMillidaltons::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microdaltons(self) -> Microdaltons<Self> {
        Quantity::new(self, UnitMicrodaltons::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanodaltons(self) -> Nanodaltons<Self> {
        Quantity::new(self, UnitNanodaltons::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picodaltons(self) -> Picodaltons<Self> {
        Quantity::new(self, UnitPicodaltons::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtodaltons(self) -> Femtodaltons<Self> {
        Quantity::new(self, UnitFemtodaltons::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attodaltons(self) -> Attodaltons<Self> {
        Quantity::new(self, UnitAttodaltons::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptodaltons(self) -> Zeptodaltons<Self> {
        Quantity::new(self, UnitZeptodaltons::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctodaltons(self) -> Yoctodaltons<Self> {
        Quantity::new(self, UnitYoctodaltons::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontodaltons(self) -> Rontodaltons<Self> {
        Quantity::new(self, UnitRontodaltons::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectodaltons(self) -> Quectodaltons<Self> {
        Quantity::new(self, UnitQuectodaltons::new())
    }


    fn as_electronvolts(self) -> Electronvolts<Self> {
        Quantity::new(self, UnitElectronvolts::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaelectronvolts(self) -> Quettaelectronvolts<Self> {
        Quantity::new(self, UnitQuettaelectronvolts::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaelectronvolts(self) -> Ronnaelectronvolts<Self> {
        Quantity::new(self, UnitRonnaelectronvolts::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaelectronvolts(self) -> Yottaelectronvolts<Self> {
        Quantity::new(self, UnitYottaelectronvolts::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaelectronvolts(self) -> Zettaelectronvolts<Self> {
        Quantity::new(self, UnitZettaelectronvolts::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaelectronvolts(self) -> Exaelectronvolts<Self> {
        Quantity::new(self, UnitExaelectronvolts::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaelectronvolts(self) -> Petaelectronvolts<Self> {
        Quantity::new(self, UnitPetaelectronvolts::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraelectronvolts(self) -> Teraelectronvolts<Self> {
        Quantity::new(self, UnitTeraelectronvolts::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaelectronvolts(self) -> Gigaelectronvolts<Self> {
        Quantity::new(self, UnitGigaelectronvolts::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaelectronvolts(self) -> Megaelectronvolts<Self> {
        Quantity::new(self, UnitMegaelectronvolts::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloelectronvolts(self) -> Kiloelectronvolts<Self> {
        Quantity::new(self, UnitKiloelectronvolts::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoelectronvolts(self) -> Hectoelectronvolts<Self> {
        Quantity::new(self, UnitHectoelectronvolts::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaelectronvolts(self) -> Decaelectronvolts<Self> {
        Quantity::new(self, UnitDecaelectronvolts::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decielectronvolts(self) -> Decielectronvolts<Self> {
        Quantity::new(self, UnitDecielectronvolts::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centielectronvolts(self) -> Centielectronvolts<Self> {
        Quantity::new(self, UnitCentielectronvolts::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millielectronvolts(self) -> Millielectronvolts<Self> {
        Quantity::new(self, UnitMillielectronvolts::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microelectronvolts(self) -> Microelectronvolts<Self> {
        Quantity::new(self, UnitMicroelectronvolts::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoelectronvolts(self) -> Nanoelectronvolts<Self> {
        Quantity::new(self, UnitNanoelectronvolts::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoelectronvolts(self) -> Picoelectronvolts<Self> {
        Quantity::new(self, UnitPicoelectronvolts::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoelectronvolts(self) -> Femtoelectronvolts<Self> {
        Quantity::new(self, UnitFemtoelectronvolts::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoelectronvolts(self) -> Attoelectronvolts<Self> {
        Quantity::new(self, UnitAttoelectronvolts::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoelectronvolts(self) -> Zeptoelectronvolts<Self> {
        Quantity::new(self, UnitZeptoelectronvolts::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoelectronvolts(self) -> Yoctoelectronvolts<Self> {
        Quantity::new(self, UnitYoctoelectronvolts::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoelectronvolts(self) -> Rontoelectronvolts<Self> {
        Quantity::new(self, UnitRontoelectronvolts::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoelectronvolts(self) -> Quectoelectronvolts<Self> {
        Quantity::new(self, UnitQuectoelectronvolts::new())
    }


    fn as_nepers(self) -> Nepers<Self> {
        Quantity::new(self, UnitNepers::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettanepers(self) -> Quettanepers<Self> {
        Quantity::new(self, UnitQuettanepers::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnanepers(self) -> Ronnanepers<Self> {
        Quantity::new(self, UnitRonnanepers::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottanepers(self) -> Yottanepers<Self> {
        Quantity::new(self, UnitYottanepers::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettanepers(self) -> Zettanepers<Self> {
        Quantity::new(self, UnitZettanepers::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exanepers(self) -> Exanepers<Self> {
        Quantity::new(self, UnitExanepers::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petanepers(self) -> Petanepers<Self> {
        Quantity::new(self, UnitPetanepers::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teranepers(self) -> Teranepers<Self> {
        Quantity::new(self, UnitTeranepers::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_giganepers(self) -> Giganepers<Self> {
        Quantity::new(self, UnitGiganepers::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_meganepers(self) -> Meganepers<Self> {
        Quantity::new(self, UnitMeganepers::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilonepers(self) -> Kilonepers<Self> {
        Quantity::new(self, UnitKilonepers::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectonepers(self) -> Hectonepers<Self> {
        Quantity::new(self, UnitHectonepers::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decanepers(self) -> Decanepers<Self> {
        Quantity::new(self, UnitDecanepers::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decinepers(self) -> Decinepers<Self> {
        Quantity::new(self, UnitDecinepers::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centinepers(self) -> Centinepers<Self> {
        Quantity::new(self, UnitCentinepers::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millinepers(self) -> Millinepers<Self> {
        Quantity::new(self, UnitMillinepers::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_micronepers(self) -> Micronepers<Self> {
        Quantity::new(self, UnitMicronepers::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanonepers(self) -> Nanonepers<Self> {
        Quantity::new(self, UnitNanonepers::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_piconepers(self) -> Piconepers<Self> {
        Quantity::new(self, UnitPiconepers::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtonepers(self) -> Femtonepers<Self> {
        Quantity::new(self, UnitFemtonepers::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attonepers(self) -> Attonepers<Self> {
        Quantity::new(self, UnitAttonepers::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptonepers(self) -> Zeptonepers<Self> {
        Quantity::new(self, UnitZeptonepers::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctonepers(self) -> Yoctonepers<Self> {
        Quantity::new(self, UnitYoctonepers::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontonepers(self) -> Rontonepers<Self> {
        Quantity::new(self, UnitRontonepers::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectonepers(self) -> Quectonepers<Self> {
        Quantity::new(self, UnitQuectonepers::new())
    }


    fn as_bels(self) -> Bels<Self> {
        Quantity::new(self, UnitBels::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettabels(self) -> Quettabels<Self> {
        Quantity::new(self, UnitQuettabels::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnabels(self) -> Ronnabels<Self> {
        Quantity::new(self, UnitRonnabels::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottabels(self) -> Yottabels<Self> {
        Quantity::new(self, UnitYottabels::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettabels(self) -> Zettabels<Self> {
        Quantity::new(self, UnitZettabels::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exabels(self) -> Exabels<Self> {
        Quantity::new(self, UnitExabels::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petabels(self) -> Petabels<Self> {
        Quantity::new(self, UnitPetabels::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terabels(self) -> Terabels<Self> {
        Quantity::new(self, UnitTerabels::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigabels(self) -> Gigabels<Self> {
        Quantity::new(self, UnitGigabels::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megabels(self) -> Megabels<Self> {
        Quantity::new(self, UnitMegabels::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilobels(self) -> Kilobels<Self> {
        Quantity::new(self, UnitKilobels::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectobels(self) -> Hectobels<Self> {
        Quantity::new(self, UnitHectobels::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decabels(self) -> Decabels<Self> {
        Quantity::new(self, UnitDecabels::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decibels(self) -> Decibels<Self> {
        Quantity::new(self, UnitDecibels::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centibels(self) -> Centibels<Self> {
        Quantity::new(self, UnitCentibels::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millibels(self) -> Millibels<Self> {
        Quantity::new(self, UnitMillibels::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microbels(self) -> Microbels<Self> {
        Quantity::new(self, UnitMicrobels::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanobels(self) -> Nanobels<Self> {
        Quantity::new(self, UnitNanobels::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picobels(self) -> Picobels<Self> {
        Quantity::new(self, UnitPicobels::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtobels(self) -> Femtobels<Self> {
        Quantity::new(self, UnitFemtobels::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attobels(self) -> Attobels<Self> {
        Quantity::new(self, UnitAttobels::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptobels(self) -> Zeptobels<Self> {
        Quantity::new(self, UnitZeptobels::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctobels(self) -> Yoctobels<Self> {
        Quantity::new(self, UnitYoctobels::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontobels(self) -> Rontobels<Self> {
        Quantity::new(self, UnitRontobels::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectobels(self) -> Quectobels<Self> {
        Quantity::new(self, UnitQuectobels::new())
    }


    fn as_atmospheres(self) -> Atmospheres<Self> {
        Quantity::new(self, UnitAtmospheres::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaatmospheres(self) -> Quettaatmospheres<Self> {
        Quantity::new(self, UnitQuettaatmospheres::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaatmospheres(self) -> Ronnaatmospheres<Self> {
        Quantity::new(self, UnitRonnaatmospheres::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaatmospheres(self) -> Yottaatmospheres<Self> {
        Quantity::new(self, UnitYottaatmospheres::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaatmospheres(self) -> Zettaatmospheres<Self> {
        Quantity::new(self, UnitZettaatmospheres::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaatmospheres(self) -> Exaatmospheres<Self> {
        Quantity::new(self, UnitExaatmospheres::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaatmospheres(self) -> Petaatmospheres<Self> {
        Quantity::new(self, UnitPetaatmospheres::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraatmospheres(self) -> Teraatmospheres<Self> {
        Quantity::new(self, UnitTeraatmospheres::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaatmospheres(self) -> Gigaatmospheres<Self> {
        Quantity::new(self, UnitGigaatmospheres::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaatmospheres(self) -> Megaatmospheres<Self> {
        Quantity::new(self, UnitMegaatmospheres::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloatmospheres(self) -> Kiloatmospheres<Self> {
        Quantity::new(self, UnitKiloatmospheres::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoatmospheres(self) -> Hectoatmospheres<Self> {
        Quantity::new(self, UnitHectoatmospheres::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaatmospheres(self) -> Decaatmospheres<Self> {
        Quantity::new(self, UnitDecaatmospheres::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciatmospheres(self) -> Deciatmospheres<Self> {
        Quantity::new(self, UnitDeciatmospheres::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiatmospheres(self) -> Centiatmospheres<Self> {
        Quantity::new(self, UnitCentiatmospheres::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliatmospheres(self) -> Milliatmospheres<Self> {
        Quantity::new(self, UnitMilliatmospheres::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microatmospheres(self) -> Microatmospheres<Self> {
        Quantity::new(self, UnitMicroatmospheres::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoatmospheres(self) -> Nanoatmospheres<Self> {
        Quantity::new(self, UnitNanoatmospheres::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoatmospheres(self) -> Picoatmospheres<Self> {
        Quantity::new(self, UnitPicoatmospheres::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoatmospheres(self) -> Femtoatmospheres<Self> {
        Quantity::new(self, UnitFemtoatmospheres::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoatmospheres(self) -> Attoatmospheres<Self> {
        Quantity::new(self, UnitAttoatmospheres::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoatmospheres(self) -> Zeptoatmospheres<Self> {
        Quantity::new(self, UnitZeptoatmospheres::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoatmospheres(self) -> Yoctoatmospheres<Self> {
        Quantity::new(self, UnitYoctoatmospheres::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoatmospheres(self) -> Rontoatmospheres<Self> {
        Quantity::new(self, UnitRontoatmospheres::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoatmospheres(self) -> Quectoatmospheres<Self> {
        Quantity::new(self, UnitQuectoatmospheres::new())
    }


    fn as_bars(self) -> Bars<Self> {
        Quantity::new(self, UnitBars::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettabars(self) -> Quettabars<Self> {
        Quantity::new(self, UnitQuettabars::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnabars(self) -> Ronnabars<Self> {
        Quantity::new(self, UnitRonnabars::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottabars(self) -> Yottabars<Self> {
        Quantity::new(self, UnitYottabars::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettabars(self) -> Zettabars<Self> {
        Quantity::new(self, UnitZettabars::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exabars(self) -> Exabars<Self> {
        Quantity::new(self, UnitExabars::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petabars(self) -> Petabars<Self> {
        Quantity::new(self, UnitPetabars::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terabars(self) -> Terabars<Self> {
        Quantity::new(self, UnitTerabars::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigabars(self) -> Gigabars<Self> {
        Quantity::new(self, UnitGigabars::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megabars(self) -> Megabars<Self> {
        Quantity::new(self, UnitMegabars::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilobars(self) -> Kilobars<Self> {
        Quantity::new(self, UnitKilobars::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectobars(self) -> Hectobars<Self> {
        Quantity::new(self, UnitHectobars::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decabars(self) -> Decabars<Self> {
        Quantity::new(self, UnitDecabars::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decibars(self) -> Decibars<Self> {
        Quantity::new(self, UnitDecibars::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centibars(self) -> Centibars<Self> {
        Quantity::new(self, UnitCentibars::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millibars(self) -> Millibars<Self> {
        Quantity::new(self, UnitMillibars::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microbars(self) -> Microbars<Self> {
        Quantity::new(self, UnitMicrobars::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanobars(self) -> Nanobars<Self> {
        Quantity::new(self, UnitNanobars::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picobars(self) -> Picobars<Self> {
        Quantity::new(self, UnitPicobars::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtobars(self) -> Femtobars<Self> {
        Quantity::new(self, UnitFemtobars::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attobars(self) -> Attobars<Self> {
        Quantity::new(self, UnitAttobars::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptobars(self) -> Zeptobars<Self> {
        Quantity::new(self, UnitZeptobars::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctobars(self) -> Yoctobars<Self> {
        Quantity::new(self, UnitYoctobars::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontobars(self) -> Rontobars<Self> {
        Quantity::new(self, UnitRontobars::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectobars(self) -> Quectobars<Self> {
        Quantity::new(self, UnitQuectobars::new())
    }


    fn as_parsec(self) -> Parsec<Self> {
        Quantity::new(self, UnitParsec::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaparsec(self) -> Quettaparsec<Self> {
        Quantity::new(self, UnitQuettaparsec::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaparsec(self) -> Ronnaparsec<Self> {
        Quantity::new(self, UnitRonnaparsec::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaparsec(self) -> Yottaparsec<Self> {
        Quantity::new(self, UnitYottaparsec::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaparsec(self) -> Zettaparsec<Self> {
        Quantity::new(self, UnitZettaparsec::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaparsec(self) -> Exaparsec<Self> {
        Quantity::new(self, UnitExaparsec::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaparsec(self) -> Petaparsec<Self> {
        Quantity::new(self, UnitPetaparsec::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraparsec(self) -> Teraparsec<Self> {
        Quantity::new(self, UnitTeraparsec::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaparsec(self) -> Gigaparsec<Self> {
        Quantity::new(self, UnitGigaparsec::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaparsec(self) -> Megaparsec<Self> {
        Quantity::new(self, UnitMegaparsec::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloparsec(self) -> Kiloparsec<Self> {
        Quantity::new(self, UnitKiloparsec::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoparsec(self) -> Hectoparsec<Self> {
        Quantity::new(self, UnitHectoparsec::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaparsec(self) -> Decaparsec<Self> {
        Quantity::new(self, UnitDecaparsec::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciparsec(self) -> Deciparsec<Self> {
        Quantity::new(self, UnitDeciparsec::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiparsec(self) -> Centiparsec<Self> {
        Quantity::new(self, UnitCentiparsec::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliparsec(self) -> Milliparsec<Self> {
        Quantity::new(self, UnitMilliparsec::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microparsec(self) -> Microparsec<Self> {
        Quantity::new(self, UnitMicroparsec::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoparsec(self) -> Nanoparsec<Self> {
        Quantity::new(self, UnitNanoparsec::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoparsec(self) -> Picoparsec<Self> {
        Quantity::new(self, UnitPicoparsec::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoparsec(self) -> Femtoparsec<Self> {
        Quantity::new(self, UnitFemtoparsec::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoparsec(self) -> Attoparsec<Self> {
        Quantity::new(self, UnitAttoparsec::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoparsec(self) -> Zeptoparsec<Self> {
        Quantity::new(self, UnitZeptoparsec::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoparsec(self) -> Yoctoparsec<Self> {
        Quantity::new(self, UnitYoctoparsec::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoparsec(self) -> Rontoparsec<Self> {
        Quantity::new(self, UnitRontoparsec::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoparsec(self) -> Quectoparsec<Self> {
        Quantity::new(self, UnitQuectoparsec::new())
    }


    fn as_millimeters_of_mercury(self) -> MillimetersOfMercury<Self> {
        Quantity::new(self, UnitMillimetersOfMercury::new())
    }
}
impl DimExt for i8 {}
impl DimExt for i16 {}
impl DimExt for i32 {}
impl DimExt for i64 {}
impl DimExt for i128 {}
impl DimExt for isize {}
impl DimExt for u8 {}
impl DimExt for u16 {}
impl DimExt for u32 {}
impl DimExt for u64 {}
impl DimExt for u128 {}
impl DimExt for usize {}
impl DimExt for f32 {}
impl DimExt for f64 {}
impl <T, D: Dimension> Quantity<T, D> {


    fn as_seconds(self) -> Seconds<T> {
        Quantity::new(self.value, UnitSeconds::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaseconds(self) -> Quettaseconds<T> {
        Quantity::new(self.value, UnitQuettaseconds::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaseconds(self) -> Ronnaseconds<T> {
        Quantity::new(self.value, UnitRonnaseconds::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaseconds(self) -> Yottaseconds<T> {
        Quantity::new(self.value, UnitYottaseconds::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaseconds(self) -> Zettaseconds<T> {
        Quantity::new(self.value, UnitZettaseconds::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaseconds(self) -> Exaseconds<T> {
        Quantity::new(self.value, UnitExaseconds::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaseconds(self) -> Petaseconds<T> {
        Quantity::new(self.value, UnitPetaseconds::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraseconds(self) -> Teraseconds<T> {
        Quantity::new(self.value, UnitTeraseconds::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaseconds(self) -> Gigaseconds<T> {
        Quantity::new(self.value, UnitGigaseconds::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaseconds(self) -> Megaseconds<T> {
        Quantity::new(self.value, UnitMegaseconds::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloseconds(self) -> Kiloseconds<T> {
        Quantity::new(self.value, UnitKiloseconds::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoseconds(self) -> Hectoseconds<T> {
        Quantity::new(self.value, UnitHectoseconds::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaseconds(self) -> Decaseconds<T> {
        Quantity::new(self.value, UnitDecaseconds::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciseconds(self) -> Deciseconds<T> {
        Quantity::new(self.value, UnitDeciseconds::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiseconds(self) -> Centiseconds<T> {
        Quantity::new(self.value, UnitCentiseconds::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliseconds(self) -> Milliseconds<T> {
        Quantity::new(self.value, UnitMilliseconds::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microseconds(self) -> Microseconds<T> {
        Quantity::new(self.value, UnitMicroseconds::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoseconds(self) -> Nanoseconds<T> {
        Quantity::new(self.value, UnitNanoseconds::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoseconds(self) -> Picoseconds<T> {
        Quantity::new(self.value, UnitPicoseconds::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoseconds(self) -> Femtoseconds<T> {
        Quantity::new(self.value, UnitFemtoseconds::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoseconds(self) -> Attoseconds<T> {
        Quantity::new(self.value, UnitAttoseconds::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoseconds(self) -> Zeptoseconds<T> {
        Quantity::new(self.value, UnitZeptoseconds::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoseconds(self) -> Yoctoseconds<T> {
        Quantity::new(self.value, UnitYoctoseconds::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoseconds(self) -> Rontoseconds<T> {
        Quantity::new(self.value, UnitRontoseconds::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoseconds(self) -> Quectoseconds<T> {
        Quantity::new(self.value, UnitQuectoseconds::new())
    }


    fn as_meters(self) -> Meters<T> {
        Quantity::new(self.value, UnitMeters::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettameters(self) -> Quettameters<T> {
        Quantity::new(self.value, UnitQuettameters::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnameters(self) -> Ronnameters<T> {
        Quantity::new(self.value, UnitRonnameters::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottameters(self) -> Yottameters<T> {
        Quantity::new(self.value, UnitYottameters::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettameters(self) -> Zettameters<T> {
        Quantity::new(self.value, UnitZettameters::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exameters(self) -> Exameters<T> {
        Quantity::new(self.value, UnitExameters::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petameters(self) -> Petameters<T> {
        Quantity::new(self.value, UnitPetameters::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terameters(self) -> Terameters<T> {
        Quantity::new(self.value, UnitTerameters::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigameters(self) -> Gigameters<T> {
        Quantity::new(self.value, UnitGigameters::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megameters(self) -> Megameters<T> {
        Quantity::new(self.value, UnitMegameters::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilometers(self) -> Kilometers<T> {
        Quantity::new(self.value, UnitKilometers::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectometers(self) -> Hectometers<T> {
        Quantity::new(self.value, UnitHectometers::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decameters(self) -> Decameters<T> {
        Quantity::new(self.value, UnitDecameters::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decimeters(self) -> Decimeters<T> {
        Quantity::new(self.value, UnitDecimeters::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centimeters(self) -> Centimeters<T> {
        Quantity::new(self.value, UnitCentimeters::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millimeters(self) -> Millimeters<T> {
        Quantity::new(self.value, UnitMillimeters::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_micrometers(self) -> Micrometers<T> {
        Quantity::new(self.value, UnitMicrometers::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanometers(self) -> Nanometers<T> {
        Quantity::new(self.value, UnitNanometers::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picometers(self) -> Picometers<T> {
        Quantity::new(self.value, UnitPicometers::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtometers(self) -> Femtometers<T> {
        Quantity::new(self.value, UnitFemtometers::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attometers(self) -> Attometers<T> {
        Quantity::new(self.value, UnitAttometers::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptometers(self) -> Zeptometers<T> {
        Quantity::new(self.value, UnitZeptometers::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctometers(self) -> Yoctometers<T> {
        Quantity::new(self.value, UnitYoctometers::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontometers(self) -> Rontometers<T> {
        Quantity::new(self.value, UnitRontometers::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectometers(self) -> Quectometers<T> {
        Quantity::new(self.value, UnitQuectometers::new())
    }


    fn as_grams(self) -> Grams<T> {
        Quantity::new(self.value, UnitGrams::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettagrams(self) -> Quettagrams<T> {
        Quantity::new(self.value, UnitQuettagrams::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnagrams(self) -> Ronnagrams<T> {
        Quantity::new(self.value, UnitRonnagrams::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottagrams(self) -> Yottagrams<T> {
        Quantity::new(self.value, UnitYottagrams::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettagrams(self) -> Zettagrams<T> {
        Quantity::new(self.value, UnitZettagrams::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exagrams(self) -> Exagrams<T> {
        Quantity::new(self.value, UnitExagrams::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petagrams(self) -> Petagrams<T> {
        Quantity::new(self.value, UnitPetagrams::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teragrams(self) -> Teragrams<T> {
        Quantity::new(self.value, UnitTeragrams::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigagrams(self) -> Gigagrams<T> {
        Quantity::new(self.value, UnitGigagrams::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megagrams(self) -> Megagrams<T> {
        Quantity::new(self.value, UnitMegagrams::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilograms(self) -> Kilograms<T> {
        Quantity::new(self.value, UnitKilograms::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectograms(self) -> Hectograms<T> {
        Quantity::new(self.value, UnitHectograms::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decagrams(self) -> Decagrams<T> {
        Quantity::new(self.value, UnitDecagrams::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decigrams(self) -> Decigrams<T> {
        Quantity::new(self.value, UnitDecigrams::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centigrams(self) -> Centigrams<T> {
        Quantity::new(self.value, UnitCentigrams::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milligrams(self) -> Milligrams<T> {
        Quantity::new(self.value, UnitMilligrams::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_micrograms(self) -> Micrograms<T> {
        Quantity::new(self.value, UnitMicrograms::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanograms(self) -> Nanograms<T> {
        Quantity::new(self.value, UnitNanograms::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picograms(self) -> Picograms<T> {
        Quantity::new(self.value, UnitPicograms::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtograms(self) -> Femtograms<T> {
        Quantity::new(self.value, UnitFemtograms::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attograms(self) -> Attograms<T> {
        Quantity::new(self.value, UnitAttograms::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptograms(self) -> Zeptograms<T> {
        Quantity::new(self.value, UnitZeptograms::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctograms(self) -> Yoctograms<T> {
        Quantity::new(self.value, UnitYoctograms::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontograms(self) -> Rontograms<T> {
        Quantity::new(self.value, UnitRontograms::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectograms(self) -> Quectograms<T> {
        Quantity::new(self.value, UnitQuectograms::new())
    }


    fn as_amperes(self) -> Amperes<T> {
        Quantity::new(self.value, UnitAmperes::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaamperes(self) -> Quettaamperes<T> {
        Quantity::new(self.value, UnitQuettaamperes::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaamperes(self) -> Ronnaamperes<T> {
        Quantity::new(self.value, UnitRonnaamperes::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaamperes(self) -> Yottaamperes<T> {
        Quantity::new(self.value, UnitYottaamperes::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaamperes(self) -> Zettaamperes<T> {
        Quantity::new(self.value, UnitZettaamperes::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaamperes(self) -> Exaamperes<T> {
        Quantity::new(self.value, UnitExaamperes::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaamperes(self) -> Petaamperes<T> {
        Quantity::new(self.value, UnitPetaamperes::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraamperes(self) -> Teraamperes<T> {
        Quantity::new(self.value, UnitTeraamperes::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaamperes(self) -> Gigaamperes<T> {
        Quantity::new(self.value, UnitGigaamperes::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaamperes(self) -> Megaamperes<T> {
        Quantity::new(self.value, UnitMegaamperes::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloamperes(self) -> Kiloamperes<T> {
        Quantity::new(self.value, UnitKiloamperes::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoamperes(self) -> Hectoamperes<T> {
        Quantity::new(self.value, UnitHectoamperes::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaamperes(self) -> Decaamperes<T> {
        Quantity::new(self.value, UnitDecaamperes::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciamperes(self) -> Deciamperes<T> {
        Quantity::new(self.value, UnitDeciamperes::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiamperes(self) -> Centiamperes<T> {
        Quantity::new(self.value, UnitCentiamperes::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliamperes(self) -> Milliamperes<T> {
        Quantity::new(self.value, UnitMilliamperes::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microamperes(self) -> Microamperes<T> {
        Quantity::new(self.value, UnitMicroamperes::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoamperes(self) -> Nanoamperes<T> {
        Quantity::new(self.value, UnitNanoamperes::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoamperes(self) -> Picoamperes<T> {
        Quantity::new(self.value, UnitPicoamperes::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoamperes(self) -> Femtoamperes<T> {
        Quantity::new(self.value, UnitFemtoamperes::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoamperes(self) -> Attoamperes<T> {
        Quantity::new(self.value, UnitAttoamperes::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoamperes(self) -> Zeptoamperes<T> {
        Quantity::new(self.value, UnitZeptoamperes::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoamperes(self) -> Yoctoamperes<T> {
        Quantity::new(self.value, UnitYoctoamperes::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoamperes(self) -> Rontoamperes<T> {
        Quantity::new(self.value, UnitRontoamperes::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoamperes(self) -> Quectoamperes<T> {
        Quantity::new(self.value, UnitQuectoamperes::new())
    }


    fn as_kelvin(self) -> Kelvin<T> {
        Quantity::new(self.value, UnitKelvin::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettakelvin(self) -> Quettakelvin<T> {
        Quantity::new(self.value, UnitQuettakelvin::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnakelvin(self) -> Ronnakelvin<T> {
        Quantity::new(self.value, UnitRonnakelvin::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottakelvin(self) -> Yottakelvin<T> {
        Quantity::new(self.value, UnitYottakelvin::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettakelvin(self) -> Zettakelvin<T> {
        Quantity::new(self.value, UnitZettakelvin::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exakelvin(self) -> Exakelvin<T> {
        Quantity::new(self.value, UnitExakelvin::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petakelvin(self) -> Petakelvin<T> {
        Quantity::new(self.value, UnitPetakelvin::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terakelvin(self) -> Terakelvin<T> {
        Quantity::new(self.value, UnitTerakelvin::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigakelvin(self) -> Gigakelvin<T> {
        Quantity::new(self.value, UnitGigakelvin::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megakelvin(self) -> Megakelvin<T> {
        Quantity::new(self.value, UnitMegakelvin::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilokelvin(self) -> Kilokelvin<T> {
        Quantity::new(self.value, UnitKilokelvin::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectokelvin(self) -> Hectokelvin<T> {
        Quantity::new(self.value, UnitHectokelvin::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decakelvin(self) -> Decakelvin<T> {
        Quantity::new(self.value, UnitDecakelvin::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decikelvin(self) -> Decikelvin<T> {
        Quantity::new(self.value, UnitDecikelvin::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centikelvin(self) -> Centikelvin<T> {
        Quantity::new(self.value, UnitCentikelvin::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millikelvin(self) -> Millikelvin<T> {
        Quantity::new(self.value, UnitMillikelvin::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microkelvin(self) -> Microkelvin<T> {
        Quantity::new(self.value, UnitMicrokelvin::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanokelvin(self) -> Nanokelvin<T> {
        Quantity::new(self.value, UnitNanokelvin::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picokelvin(self) -> Picokelvin<T> {
        Quantity::new(self.value, UnitPicokelvin::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtokelvin(self) -> Femtokelvin<T> {
        Quantity::new(self.value, UnitFemtokelvin::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attokelvin(self) -> Attokelvin<T> {
        Quantity::new(self.value, UnitAttokelvin::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptokelvin(self) -> Zeptokelvin<T> {
        Quantity::new(self.value, UnitZeptokelvin::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctokelvin(self) -> Yoctokelvin<T> {
        Quantity::new(self.value, UnitYoctokelvin::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontokelvin(self) -> Rontokelvin<T> {
        Quantity::new(self.value, UnitRontokelvin::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectokelvin(self) -> Quectokelvin<T> {
        Quantity::new(self.value, UnitQuectokelvin::new())
    }


    fn as_moles(self) -> Moles<T> {
        Quantity::new(self.value, UnitMoles::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettamoles(self) -> Quettamoles<T> {
        Quantity::new(self.value, UnitQuettamoles::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnamoles(self) -> Ronnamoles<T> {
        Quantity::new(self.value, UnitRonnamoles::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottamoles(self) -> Yottamoles<T> {
        Quantity::new(self.value, UnitYottamoles::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettamoles(self) -> Zettamoles<T> {
        Quantity::new(self.value, UnitZettamoles::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_examoles(self) -> Examoles<T> {
        Quantity::new(self.value, UnitExamoles::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petamoles(self) -> Petamoles<T> {
        Quantity::new(self.value, UnitPetamoles::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teramoles(self) -> Teramoles<T> {
        Quantity::new(self.value, UnitTeramoles::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigamoles(self) -> Gigamoles<T> {
        Quantity::new(self.value, UnitGigamoles::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megamoles(self) -> Megamoles<T> {
        Quantity::new(self.value, UnitMegamoles::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilomoles(self) -> Kilomoles<T> {
        Quantity::new(self.value, UnitKilomoles::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectomoles(self) -> Hectomoles<T> {
        Quantity::new(self.value, UnitHectomoles::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decamoles(self) -> Decamoles<T> {
        Quantity::new(self.value, UnitDecamoles::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decimoles(self) -> Decimoles<T> {
        Quantity::new(self.value, UnitDecimoles::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centimoles(self) -> Centimoles<T> {
        Quantity::new(self.value, UnitCentimoles::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millimoles(self) -> Millimoles<T> {
        Quantity::new(self.value, UnitMillimoles::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_micromoles(self) -> Micromoles<T> {
        Quantity::new(self.value, UnitMicromoles::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanomoles(self) -> Nanomoles<T> {
        Quantity::new(self.value, UnitNanomoles::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picomoles(self) -> Picomoles<T> {
        Quantity::new(self.value, UnitPicomoles::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtomoles(self) -> Femtomoles<T> {
        Quantity::new(self.value, UnitFemtomoles::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attomoles(self) -> Attomoles<T> {
        Quantity::new(self.value, UnitAttomoles::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptomoles(self) -> Zeptomoles<T> {
        Quantity::new(self.value, UnitZeptomoles::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctomoles(self) -> Yoctomoles<T> {
        Quantity::new(self.value, UnitYoctomoles::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontomoles(self) -> Rontomoles<T> {
        Quantity::new(self.value, UnitRontomoles::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectomoles(self) -> Quectomoles<T> {
        Quantity::new(self.value, UnitQuectomoles::new())
    }


    fn as_candelas(self) -> Candelas<T> {
        Quantity::new(self.value, UnitCandelas::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettacandelas(self) -> Quettacandelas<T> {
        Quantity::new(self.value, UnitQuettacandelas::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnacandelas(self) -> Ronnacandelas<T> {
        Quantity::new(self.value, UnitRonnacandelas::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottacandelas(self) -> Yottacandelas<T> {
        Quantity::new(self.value, UnitYottacandelas::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettacandelas(self) -> Zettacandelas<T> {
        Quantity::new(self.value, UnitZettacandelas::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exacandelas(self) -> Exacandelas<T> {
        Quantity::new(self.value, UnitExacandelas::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petacandelas(self) -> Petacandelas<T> {
        Quantity::new(self.value, UnitPetacandelas::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teracandelas(self) -> Teracandelas<T> {
        Quantity::new(self.value, UnitTeracandelas::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigacandelas(self) -> Gigacandelas<T> {
        Quantity::new(self.value, UnitGigacandelas::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megacandelas(self) -> Megacandelas<T> {
        Quantity::new(self.value, UnitMegacandelas::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilocandelas(self) -> Kilocandelas<T> {
        Quantity::new(self.value, UnitKilocandelas::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectocandelas(self) -> Hectocandelas<T> {
        Quantity::new(self.value, UnitHectocandelas::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decacandelas(self) -> Decacandelas<T> {
        Quantity::new(self.value, UnitDecacandelas::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decicandelas(self) -> Decicandelas<T> {
        Quantity::new(self.value, UnitDecicandelas::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centicandelas(self) -> Centicandelas<T> {
        Quantity::new(self.value, UnitCenticandelas::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millicandelas(self) -> Millicandelas<T> {
        Quantity::new(self.value, UnitMillicandelas::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microcandelas(self) -> Microcandelas<T> {
        Quantity::new(self.value, UnitMicrocandelas::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanocandelas(self) -> Nanocandelas<T> {
        Quantity::new(self.value, UnitNanocandelas::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picocandelas(self) -> Picocandelas<T> {
        Quantity::new(self.value, UnitPicocandelas::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtocandelas(self) -> Femtocandelas<T> {
        Quantity::new(self.value, UnitFemtocandelas::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attocandelas(self) -> Attocandelas<T> {
        Quantity::new(self.value, UnitAttocandelas::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptocandelas(self) -> Zeptocandelas<T> {
        Quantity::new(self.value, UnitZeptocandelas::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctocandelas(self) -> Yoctocandelas<T> {
        Quantity::new(self.value, UnitYoctocandelas::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontocandelas(self) -> Rontocandelas<T> {
        Quantity::new(self.value, UnitRontocandelas::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectocandelas(self) -> Quectocandelas<T> {
        Quantity::new(self.value, UnitQuectocandelas::new())
    }


    fn as_byte(self) -> Byte<T> {
        Quantity::new(self.value, UnitByte::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettabyte(self) -> Quettabyte<T> {
        Quantity::new(self.value, UnitQuettabyte::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnabyte(self) -> Ronnabyte<T> {
        Quantity::new(self.value, UnitRonnabyte::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottabyte(self) -> Yottabyte<T> {
        Quantity::new(self.value, UnitYottabyte::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettabyte(self) -> Zettabyte<T> {
        Quantity::new(self.value, UnitZettabyte::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exabyte(self) -> Exabyte<T> {
        Quantity::new(self.value, UnitExabyte::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petabyte(self) -> Petabyte<T> {
        Quantity::new(self.value, UnitPetabyte::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terabyte(self) -> Terabyte<T> {
        Quantity::new(self.value, UnitTerabyte::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigabyte(self) -> Gigabyte<T> {
        Quantity::new(self.value, UnitGigabyte::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megabyte(self) -> Megabyte<T> {
        Quantity::new(self.value, UnitMegabyte::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilobyte(self) -> Kilobyte<T> {
        Quantity::new(self.value, UnitKilobyte::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectobyte(self) -> Hectobyte<T> {
        Quantity::new(self.value, UnitHectobyte::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decabyte(self) -> Decabyte<T> {
        Quantity::new(self.value, UnitDecabyte::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decibyte(self) -> Decibyte<T> {
        Quantity::new(self.value, UnitDecibyte::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centibyte(self) -> Centibyte<T> {
        Quantity::new(self.value, UnitCentibyte::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millibyte(self) -> Millibyte<T> {
        Quantity::new(self.value, UnitMillibyte::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microbyte(self) -> Microbyte<T> {
        Quantity::new(self.value, UnitMicrobyte::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanobyte(self) -> Nanobyte<T> {
        Quantity::new(self.value, UnitNanobyte::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picobyte(self) -> Picobyte<T> {
        Quantity::new(self.value, UnitPicobyte::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtobyte(self) -> Femtobyte<T> {
        Quantity::new(self.value, UnitFemtobyte::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attobyte(self) -> Attobyte<T> {
        Quantity::new(self.value, UnitAttobyte::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptobyte(self) -> Zeptobyte<T> {
        Quantity::new(self.value, UnitZeptobyte::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctobyte(self) -> Yoctobyte<T> {
        Quantity::new(self.value, UnitYoctobyte::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontobyte(self) -> Rontobyte<T> {
        Quantity::new(self.value, UnitRontobyte::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectobyte(self) -> Quectobyte<T> {
        Quantity::new(self.value, UnitQuectobyte::new())
    }


    fn as_radians(self) -> Radians<T> {
        Quantity::new(self.value, UnitRadians::new())
    }


    fn as_steradians(self) -> Steradians<T> {
        Quantity::new(self.value, UnitSteradians::new())
    }


    fn as_celsius(self) -> Celsius<T> {
        Quantity::new(self.value, UnitCelsius::new())
    }


    fn as_minutes(self) -> Minutes<T> {
        Quantity::new(self.value, UnitMinutes::new())
    }


    fn as_hours(self) -> Hours<T> {
        Quantity::new(self.value, UnitHours::new())
    }


    fn as_days(self) -> Days<T> {
        Quantity::new(self.value, UnitDays::new())
    }


    fn as_astronomical_units(self) -> AstronomicalUnits<T> {
        Quantity::new(self.value, UnitAstronomicalUnits::new())
    }


    fn as_degrees(self) -> Degrees<T> {
        Quantity::new(self.value, UnitDegrees::new())
    }


    fn as_arcminutes(self) -> Arcminutes<T> {
        Quantity::new(self.value, UnitArcminutes::new())
    }


    fn as_arcseconds(self) -> Arcseconds<T> {
        Quantity::new(self.value, UnitArcseconds::new())
    }


    fn as_ares(self) -> Ares<T> {
        Quantity::new(self.value, UnitAres::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaares(self) -> Quettaares<T> {
        Quantity::new(self.value, UnitQuettaares::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaares(self) -> Ronnaares<T> {
        Quantity::new(self.value, UnitRonnaares::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaares(self) -> Yottaares<T> {
        Quantity::new(self.value, UnitYottaares::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaares(self) -> Zettaares<T> {
        Quantity::new(self.value, UnitZettaares::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaares(self) -> Exaares<T> {
        Quantity::new(self.value, UnitExaares::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaares(self) -> Petaares<T> {
        Quantity::new(self.value, UnitPetaares::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraares(self) -> Teraares<T> {
        Quantity::new(self.value, UnitTeraares::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaares(self) -> Gigaares<T> {
        Quantity::new(self.value, UnitGigaares::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaares(self) -> Megaares<T> {
        Quantity::new(self.value, UnitMegaares::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloares(self) -> Kiloares<T> {
        Quantity::new(self.value, UnitKiloares::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoares(self) -> Hectoares<T> {
        Quantity::new(self.value, UnitHectoares::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaares(self) -> Decaares<T> {
        Quantity::new(self.value, UnitDecaares::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciares(self) -> Deciares<T> {
        Quantity::new(self.value, UnitDeciares::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiares(self) -> Centiares<T> {
        Quantity::new(self.value, UnitCentiares::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliares(self) -> Milliares<T> {
        Quantity::new(self.value, UnitMilliares::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microares(self) -> Microares<T> {
        Quantity::new(self.value, UnitMicroares::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoares(self) -> Nanoares<T> {
        Quantity::new(self.value, UnitNanoares::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoares(self) -> Picoares<T> {
        Quantity::new(self.value, UnitPicoares::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoares(self) -> Femtoares<T> {
        Quantity::new(self.value, UnitFemtoares::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoares(self) -> Attoares<T> {
        Quantity::new(self.value, UnitAttoares::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoares(self) -> Zeptoares<T> {
        Quantity::new(self.value, UnitZeptoares::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoares(self) -> Yoctoares<T> {
        Quantity::new(self.value, UnitYoctoares::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoares(self) -> Rontoares<T> {
        Quantity::new(self.value, UnitRontoares::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoares(self) -> Quectoares<T> {
        Quantity::new(self.value, UnitQuectoares::new())
    }


    fn as_liters(self) -> Liters<T> {
        Quantity::new(self.value, UnitLiters::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaliters(self) -> Quettaliters<T> {
        Quantity::new(self.value, UnitQuettaliters::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaliters(self) -> Ronnaliters<T> {
        Quantity::new(self.value, UnitRonnaliters::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaliters(self) -> Yottaliters<T> {
        Quantity::new(self.value, UnitYottaliters::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaliters(self) -> Zettaliters<T> {
        Quantity::new(self.value, UnitZettaliters::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaliters(self) -> Exaliters<T> {
        Quantity::new(self.value, UnitExaliters::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaliters(self) -> Petaliters<T> {
        Quantity::new(self.value, UnitPetaliters::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraliters(self) -> Teraliters<T> {
        Quantity::new(self.value, UnitTeraliters::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaliters(self) -> Gigaliters<T> {
        Quantity::new(self.value, UnitGigaliters::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaliters(self) -> Megaliters<T> {
        Quantity::new(self.value, UnitMegaliters::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloliters(self) -> Kiloliters<T> {
        Quantity::new(self.value, UnitKiloliters::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoliters(self) -> Hectoliters<T> {
        Quantity::new(self.value, UnitHectoliters::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaliters(self) -> Decaliters<T> {
        Quantity::new(self.value, UnitDecaliters::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciliters(self) -> Deciliters<T> {
        Quantity::new(self.value, UnitDeciliters::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiliters(self) -> Centiliters<T> {
        Quantity::new(self.value, UnitCentiliters::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliliters(self) -> Milliliters<T> {
        Quantity::new(self.value, UnitMilliliters::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microliters(self) -> Microliters<T> {
        Quantity::new(self.value, UnitMicroliters::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoliters(self) -> Nanoliters<T> {
        Quantity::new(self.value, UnitNanoliters::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoliters(self) -> Picoliters<T> {
        Quantity::new(self.value, UnitPicoliters::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoliters(self) -> Femtoliters<T> {
        Quantity::new(self.value, UnitFemtoliters::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoliters(self) -> Attoliters<T> {
        Quantity::new(self.value, UnitAttoliters::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoliters(self) -> Zeptoliters<T> {
        Quantity::new(self.value, UnitZeptoliters::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoliters(self) -> Yoctoliters<T> {
        Quantity::new(self.value, UnitYoctoliters::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoliters(self) -> Rontoliters<T> {
        Quantity::new(self.value, UnitRontoliters::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoliters(self) -> Quectoliters<T> {
        Quantity::new(self.value, UnitQuectoliters::new())
    }


    fn as_daltons(self) -> Daltons<T> {
        Quantity::new(self.value, UnitDaltons::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettadaltons(self) -> Quettadaltons<T> {
        Quantity::new(self.value, UnitQuettadaltons::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnadaltons(self) -> Ronnadaltons<T> {
        Quantity::new(self.value, UnitRonnadaltons::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottadaltons(self) -> Yottadaltons<T> {
        Quantity::new(self.value, UnitYottadaltons::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettadaltons(self) -> Zettadaltons<T> {
        Quantity::new(self.value, UnitZettadaltons::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exadaltons(self) -> Exadaltons<T> {
        Quantity::new(self.value, UnitExadaltons::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petadaltons(self) -> Petadaltons<T> {
        Quantity::new(self.value, UnitPetadaltons::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teradaltons(self) -> Teradaltons<T> {
        Quantity::new(self.value, UnitTeradaltons::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigadaltons(self) -> Gigadaltons<T> {
        Quantity::new(self.value, UnitGigadaltons::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megadaltons(self) -> Megadaltons<T> {
        Quantity::new(self.value, UnitMegadaltons::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilodaltons(self) -> Kilodaltons<T> {
        Quantity::new(self.value, UnitKilodaltons::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectodaltons(self) -> Hectodaltons<T> {
        Quantity::new(self.value, UnitHectodaltons::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decadaltons(self) -> Decadaltons<T> {
        Quantity::new(self.value, UnitDecadaltons::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decidaltons(self) -> Decidaltons<T> {
        Quantity::new(self.value, UnitDecidaltons::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centidaltons(self) -> Centidaltons<T> {
        Quantity::new(self.value, UnitCentidaltons::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millidaltons(self) -> Millidaltons<T> {
        Quantity::new(self.value, UnitMillidaltons::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microdaltons(self) -> Microdaltons<T> {
        Quantity::new(self.value, UnitMicrodaltons::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanodaltons(self) -> Nanodaltons<T> {
        Quantity::new(self.value, UnitNanodaltons::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picodaltons(self) -> Picodaltons<T> {
        Quantity::new(self.value, UnitPicodaltons::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtodaltons(self) -> Femtodaltons<T> {
        Quantity::new(self.value, UnitFemtodaltons::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attodaltons(self) -> Attodaltons<T> {
        Quantity::new(self.value, UnitAttodaltons::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptodaltons(self) -> Zeptodaltons<T> {
        Quantity::new(self.value, UnitZeptodaltons::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctodaltons(self) -> Yoctodaltons<T> {
        Quantity::new(self.value, UnitYoctodaltons::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontodaltons(self) -> Rontodaltons<T> {
        Quantity::new(self.value, UnitRontodaltons::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectodaltons(self) -> Quectodaltons<T> {
        Quantity::new(self.value, UnitQuectodaltons::new())
    }


    fn as_electronvolts(self) -> Electronvolts<T> {
        Quantity::new(self.value, UnitElectronvolts::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaelectronvolts(self) -> Quettaelectronvolts<T> {
        Quantity::new(self.value, UnitQuettaelectronvolts::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaelectronvolts(self) -> Ronnaelectronvolts<T> {
        Quantity::new(self.value, UnitRonnaelectronvolts::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaelectronvolts(self) -> Yottaelectronvolts<T> {
        Quantity::new(self.value, UnitYottaelectronvolts::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaelectronvolts(self) -> Zettaelectronvolts<T> {
        Quantity::new(self.value, UnitZettaelectronvolts::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaelectronvolts(self) -> Exaelectronvolts<T> {
        Quantity::new(self.value, UnitExaelectronvolts::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaelectronvolts(self) -> Petaelectronvolts<T> {
        Quantity::new(self.value, UnitPetaelectronvolts::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraelectronvolts(self) -> Teraelectronvolts<T> {
        Quantity::new(self.value, UnitTeraelectronvolts::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaelectronvolts(self) -> Gigaelectronvolts<T> {
        Quantity::new(self.value, UnitGigaelectronvolts::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaelectronvolts(self) -> Megaelectronvolts<T> {
        Quantity::new(self.value, UnitMegaelectronvolts::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloelectronvolts(self) -> Kiloelectronvolts<T> {
        Quantity::new(self.value, UnitKiloelectronvolts::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoelectronvolts(self) -> Hectoelectronvolts<T> {
        Quantity::new(self.value, UnitHectoelectronvolts::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaelectronvolts(self) -> Decaelectronvolts<T> {
        Quantity::new(self.value, UnitDecaelectronvolts::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decielectronvolts(self) -> Decielectronvolts<T> {
        Quantity::new(self.value, UnitDecielectronvolts::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centielectronvolts(self) -> Centielectronvolts<T> {
        Quantity::new(self.value, UnitCentielectronvolts::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millielectronvolts(self) -> Millielectronvolts<T> {
        Quantity::new(self.value, UnitMillielectronvolts::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microelectronvolts(self) -> Microelectronvolts<T> {
        Quantity::new(self.value, UnitMicroelectronvolts::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoelectronvolts(self) -> Nanoelectronvolts<T> {
        Quantity::new(self.value, UnitNanoelectronvolts::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoelectronvolts(self) -> Picoelectronvolts<T> {
        Quantity::new(self.value, UnitPicoelectronvolts::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoelectronvolts(self) -> Femtoelectronvolts<T> {
        Quantity::new(self.value, UnitFemtoelectronvolts::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoelectronvolts(self) -> Attoelectronvolts<T> {
        Quantity::new(self.value, UnitAttoelectronvolts::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoelectronvolts(self) -> Zeptoelectronvolts<T> {
        Quantity::new(self.value, UnitZeptoelectronvolts::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoelectronvolts(self) -> Yoctoelectronvolts<T> {
        Quantity::new(self.value, UnitYoctoelectronvolts::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoelectronvolts(self) -> Rontoelectronvolts<T> {
        Quantity::new(self.value, UnitRontoelectronvolts::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoelectronvolts(self) -> Quectoelectronvolts<T> {
        Quantity::new(self.value, UnitQuectoelectronvolts::new())
    }


    fn as_nepers(self) -> Nepers<T> {
        Quantity::new(self.value, UnitNepers::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettanepers(self) -> Quettanepers<T> {
        Quantity::new(self.value, UnitQuettanepers::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnanepers(self) -> Ronnanepers<T> {
        Quantity::new(self.value, UnitRonnanepers::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottanepers(self) -> Yottanepers<T> {
        Quantity::new(self.value, UnitYottanepers::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettanepers(self) -> Zettanepers<T> {
        Quantity::new(self.value, UnitZettanepers::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exanepers(self) -> Exanepers<T> {
        Quantity::new(self.value, UnitExanepers::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petanepers(self) -> Petanepers<T> {
        Quantity::new(self.value, UnitPetanepers::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teranepers(self) -> Teranepers<T> {
        Quantity::new(self.value, UnitTeranepers::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_giganepers(self) -> Giganepers<T> {
        Quantity::new(self.value, UnitGiganepers::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_meganepers(self) -> Meganepers<T> {
        Quantity::new(self.value, UnitMeganepers::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilonepers(self) -> Kilonepers<T> {
        Quantity::new(self.value, UnitKilonepers::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectonepers(self) -> Hectonepers<T> {
        Quantity::new(self.value, UnitHectonepers::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decanepers(self) -> Decanepers<T> {
        Quantity::new(self.value, UnitDecanepers::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decinepers(self) -> Decinepers<T> {
        Quantity::new(self.value, UnitDecinepers::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centinepers(self) -> Centinepers<T> {
        Quantity::new(self.value, UnitCentinepers::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millinepers(self) -> Millinepers<T> {
        Quantity::new(self.value, UnitMillinepers::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_micronepers(self) -> Micronepers<T> {
        Quantity::new(self.value, UnitMicronepers::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanonepers(self) -> Nanonepers<T> {
        Quantity::new(self.value, UnitNanonepers::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_piconepers(self) -> Piconepers<T> {
        Quantity::new(self.value, UnitPiconepers::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtonepers(self) -> Femtonepers<T> {
        Quantity::new(self.value, UnitFemtonepers::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attonepers(self) -> Attonepers<T> {
        Quantity::new(self.value, UnitAttonepers::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptonepers(self) -> Zeptonepers<T> {
        Quantity::new(self.value, UnitZeptonepers::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctonepers(self) -> Yoctonepers<T> {
        Quantity::new(self.value, UnitYoctonepers::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontonepers(self) -> Rontonepers<T> {
        Quantity::new(self.value, UnitRontonepers::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectonepers(self) -> Quectonepers<T> {
        Quantity::new(self.value, UnitQuectonepers::new())
    }


    fn as_bels(self) -> Bels<T> {
        Quantity::new(self.value, UnitBels::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettabels(self) -> Quettabels<T> {
        Quantity::new(self.value, UnitQuettabels::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnabels(self) -> Ronnabels<T> {
        Quantity::new(self.value, UnitRonnabels::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottabels(self) -> Yottabels<T> {
        Quantity::new(self.value, UnitYottabels::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettabels(self) -> Zettabels<T> {
        Quantity::new(self.value, UnitZettabels::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exabels(self) -> Exabels<T> {
        Quantity::new(self.value, UnitExabels::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petabels(self) -> Petabels<T> {
        Quantity::new(self.value, UnitPetabels::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terabels(self) -> Terabels<T> {
        Quantity::new(self.value, UnitTerabels::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigabels(self) -> Gigabels<T> {
        Quantity::new(self.value, UnitGigabels::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megabels(self) -> Megabels<T> {
        Quantity::new(self.value, UnitMegabels::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilobels(self) -> Kilobels<T> {
        Quantity::new(self.value, UnitKilobels::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectobels(self) -> Hectobels<T> {
        Quantity::new(self.value, UnitHectobels::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decabels(self) -> Decabels<T> {
        Quantity::new(self.value, UnitDecabels::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decibels(self) -> Decibels<T> {
        Quantity::new(self.value, UnitDecibels::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centibels(self) -> Centibels<T> {
        Quantity::new(self.value, UnitCentibels::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millibels(self) -> Millibels<T> {
        Quantity::new(self.value, UnitMillibels::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microbels(self) -> Microbels<T> {
        Quantity::new(self.value, UnitMicrobels::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanobels(self) -> Nanobels<T> {
        Quantity::new(self.value, UnitNanobels::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picobels(self) -> Picobels<T> {
        Quantity::new(self.value, UnitPicobels::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtobels(self) -> Femtobels<T> {
        Quantity::new(self.value, UnitFemtobels::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attobels(self) -> Attobels<T> {
        Quantity::new(self.value, UnitAttobels::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptobels(self) -> Zeptobels<T> {
        Quantity::new(self.value, UnitZeptobels::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctobels(self) -> Yoctobels<T> {
        Quantity::new(self.value, UnitYoctobels::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontobels(self) -> Rontobels<T> {
        Quantity::new(self.value, UnitRontobels::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectobels(self) -> Quectobels<T> {
        Quantity::new(self.value, UnitQuectobels::new())
    }


    fn as_atmospheres(self) -> Atmospheres<T> {
        Quantity::new(self.value, UnitAtmospheres::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaatmospheres(self) -> Quettaatmospheres<T> {
        Quantity::new(self.value, UnitQuettaatmospheres::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaatmospheres(self) -> Ronnaatmospheres<T> {
        Quantity::new(self.value, UnitRonnaatmospheres::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaatmospheres(self) -> Yottaatmospheres<T> {
        Quantity::new(self.value, UnitYottaatmospheres::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaatmospheres(self) -> Zettaatmospheres<T> {
        Quantity::new(self.value, UnitZettaatmospheres::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaatmospheres(self) -> Exaatmospheres<T> {
        Quantity::new(self.value, UnitExaatmospheres::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaatmospheres(self) -> Petaatmospheres<T> {
        Quantity::new(self.value, UnitPetaatmospheres::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraatmospheres(self) -> Teraatmospheres<T> {
        Quantity::new(self.value, UnitTeraatmospheres::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaatmospheres(self) -> Gigaatmospheres<T> {
        Quantity::new(self.value, UnitGigaatmospheres::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaatmospheres(self) -> Megaatmospheres<T> {
        Quantity::new(self.value, UnitMegaatmospheres::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloatmospheres(self) -> Kiloatmospheres<T> {
        Quantity::new(self.value, UnitKiloatmospheres::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoatmospheres(self) -> Hectoatmospheres<T> {
        Quantity::new(self.value, UnitHectoatmospheres::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaatmospheres(self) -> Decaatmospheres<T> {
        Quantity::new(self.value, UnitDecaatmospheres::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciatmospheres(self) -> Deciatmospheres<T> {
        Quantity::new(self.value, UnitDeciatmospheres::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiatmospheres(self) -> Centiatmospheres<T> {
        Quantity::new(self.value, UnitCentiatmospheres::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliatmospheres(self) -> Milliatmospheres<T> {
        Quantity::new(self.value, UnitMilliatmospheres::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microatmospheres(self) -> Microatmospheres<T> {
        Quantity::new(self.value, UnitMicroatmospheres::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoatmospheres(self) -> Nanoatmospheres<T> {
        Quantity::new(self.value, UnitNanoatmospheres::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoatmospheres(self) -> Picoatmospheres<T> {
        Quantity::new(self.value, UnitPicoatmospheres::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoatmospheres(self) -> Femtoatmospheres<T> {
        Quantity::new(self.value, UnitFemtoatmospheres::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoatmospheres(self) -> Attoatmospheres<T> {
        Quantity::new(self.value, UnitAttoatmospheres::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoatmospheres(self) -> Zeptoatmospheres<T> {
        Quantity::new(self.value, UnitZeptoatmospheres::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoatmospheres(self) -> Yoctoatmospheres<T> {
        Quantity::new(self.value, UnitYoctoatmospheres::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoatmospheres(self) -> Rontoatmospheres<T> {
        Quantity::new(self.value, UnitRontoatmospheres::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoatmospheres(self) -> Quectoatmospheres<T> {
        Quantity::new(self.value, UnitQuectoatmospheres::new())
    }


    fn as_bars(self) -> Bars<T> {
        Quantity::new(self.value, UnitBars::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettabars(self) -> Quettabars<T> {
        Quantity::new(self.value, UnitQuettabars::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnabars(self) -> Ronnabars<T> {
        Quantity::new(self.value, UnitRonnabars::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottabars(self) -> Yottabars<T> {
        Quantity::new(self.value, UnitYottabars::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettabars(self) -> Zettabars<T> {
        Quantity::new(self.value, UnitZettabars::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exabars(self) -> Exabars<T> {
        Quantity::new(self.value, UnitExabars::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petabars(self) -> Petabars<T> {
        Quantity::new(self.value, UnitPetabars::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_terabars(self) -> Terabars<T> {
        Quantity::new(self.value, UnitTerabars::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigabars(self) -> Gigabars<T> {
        Quantity::new(self.value, UnitGigabars::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megabars(self) -> Megabars<T> {
        Quantity::new(self.value, UnitMegabars::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kilobars(self) -> Kilobars<T> {
        Quantity::new(self.value, UnitKilobars::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectobars(self) -> Hectobars<T> {
        Quantity::new(self.value, UnitHectobars::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decabars(self) -> Decabars<T> {
        Quantity::new(self.value, UnitDecabars::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_decibars(self) -> Decibars<T> {
        Quantity::new(self.value, UnitDecibars::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centibars(self) -> Centibars<T> {
        Quantity::new(self.value, UnitCentibars::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_millibars(self) -> Millibars<T> {
        Quantity::new(self.value, UnitMillibars::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microbars(self) -> Microbars<T> {
        Quantity::new(self.value, UnitMicrobars::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanobars(self) -> Nanobars<T> {
        Quantity::new(self.value, UnitNanobars::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picobars(self) -> Picobars<T> {
        Quantity::new(self.value, UnitPicobars::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtobars(self) -> Femtobars<T> {
        Quantity::new(self.value, UnitFemtobars::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attobars(self) -> Attobars<T> {
        Quantity::new(self.value, UnitAttobars::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptobars(self) -> Zeptobars<T> {
        Quantity::new(self.value, UnitZeptobars::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctobars(self) -> Yoctobars<T> {
        Quantity::new(self.value, UnitYoctobars::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontobars(self) -> Rontobars<T> {
        Quantity::new(self.value, UnitRontobars::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectobars(self) -> Quectobars<T> {
        Quantity::new(self.value, UnitQuectobars::new())
    }


    fn as_parsec(self) -> Parsec<T> {
        Quantity::new(self.value, UnitParsec::new())
    }

    #[cfg(feature = "si_quetta")]
    fn as_quettaparsec(self) -> Quettaparsec<T> {
        Quantity::new(self.value, UnitQuettaparsec::new())
    }

    #[cfg(feature = "si_ronna")]
    fn as_ronnaparsec(self) -> Ronnaparsec<T> {
        Quantity::new(self.value, UnitRonnaparsec::new())
    }

    #[cfg(feature = "si_yotta")]
    fn as_yottaparsec(self) -> Yottaparsec<T> {
        Quantity::new(self.value, UnitYottaparsec::new())
    }

    #[cfg(feature = "si_zetta")]
    fn as_zettaparsec(self) -> Zettaparsec<T> {
        Quantity::new(self.value, UnitZettaparsec::new())
    }

    #[cfg(feature = "si_exa")]
    fn as_exaparsec(self) -> Exaparsec<T> {
        Quantity::new(self.value, UnitExaparsec::new())
    }

    #[cfg(feature = "si_peta")]
    fn as_petaparsec(self) -> Petaparsec<T> {
        Quantity::new(self.value, UnitPetaparsec::new())
    }

    #[cfg(feature = "si_tera")]
    fn as_teraparsec(self) -> Teraparsec<T> {
        Quantity::new(self.value, UnitTeraparsec::new())
    }

    #[cfg(feature = "si_giga")]
    fn as_gigaparsec(self) -> Gigaparsec<T> {
        Quantity::new(self.value, UnitGigaparsec::new())
    }

    #[cfg(feature = "si_mega")]
    fn as_megaparsec(self) -> Megaparsec<T> {
        Quantity::new(self.value, UnitMegaparsec::new())
    }

    #[cfg(feature = "si_kilo")]
    fn as_kiloparsec(self) -> Kiloparsec<T> {
        Quantity::new(self.value, UnitKiloparsec::new())
    }

    #[cfg(feature = "si_hecto")]
    fn as_hectoparsec(self) -> Hectoparsec<T> {
        Quantity::new(self.value, UnitHectoparsec::new())
    }

    #[cfg(feature = "si_deca")]
    fn as_decaparsec(self) -> Decaparsec<T> {
        Quantity::new(self.value, UnitDecaparsec::new())
    }

    #[cfg(feature = "si_deci")]
    fn as_deciparsec(self) -> Deciparsec<T> {
        Quantity::new(self.value, UnitDeciparsec::new())
    }

    #[cfg(feature = "si_centi")]
    fn as_centiparsec(self) -> Centiparsec<T> {
        Quantity::new(self.value, UnitCentiparsec::new())
    }

    #[cfg(feature = "si_milli")]
    fn as_milliparsec(self) -> Milliparsec<T> {
        Quantity::new(self.value, UnitMilliparsec::new())
    }

    #[cfg(feature = "si_micro")]
    fn as_microparsec(self) -> Microparsec<T> {
        Quantity::new(self.value, UnitMicroparsec::new())
    }

    #[cfg(feature = "si_nano")]
    fn as_nanoparsec(self) -> Nanoparsec<T> {
        Quantity::new(self.value, UnitNanoparsec::new())
    }

    #[cfg(feature = "si_pico")]
    fn as_picoparsec(self) -> Picoparsec<T> {
        Quantity::new(self.value, UnitPicoparsec::new())
    }

    #[cfg(feature = "si_femto")]
    fn as_femtoparsec(self) -> Femtoparsec<T> {
        Quantity::new(self.value, UnitFemtoparsec::new())
    }

    #[cfg(feature = "si_atto")]
    fn as_attoparsec(self) -> Attoparsec<T> {
        Quantity::new(self.value, UnitAttoparsec::new())
    }

    #[cfg(feature = "si_zepto")]
    fn as_zeptoparsec(self) -> Zeptoparsec<T> {
        Quantity::new(self.value, UnitZeptoparsec::new())
    }

    #[cfg(feature = "si_yocto")]
    fn as_yoctoparsec(self) -> Yoctoparsec<T> {
        Quantity::new(self.value, UnitYoctoparsec::new())
    }

    #[cfg(feature = "si_ronto")]
    fn as_rontoparsec(self) -> Rontoparsec<T> {
        Quantity::new(self.value, UnitRontoparsec::new())
    }

    #[cfg(feature = "si_quecto")]
    fn as_quectoparsec(self) -> Quectoparsec<T> {
        Quantity::new(self.value, UnitQuectoparsec::new())
    }


    fn as_millimeters_of_mercury(self) -> MillimetersOfMercury<T> {
        Quantity::new(self.value, UnitMillimetersOfMercury::new())
    }
}