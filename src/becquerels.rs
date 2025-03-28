use crate::*;
pub type UnitBecquerels = DimensionStruct<Z0, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Becquerels<T> = Quantity<T, UnitBecquerels>;
pub type UnitQuettabecquerels = DimensionStruct<P30, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Quettabecquerels<T> = Quantity<T, UnitQuettabecquerels>;
pub type UnitRonnabecquerels = DimensionStruct<P27, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Ronnabecquerels<T> = Quantity<T, UnitRonnabecquerels>;
pub type UnitYottabecquerels = DimensionStruct<P24, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Yottabecquerels<T> = Quantity<T, UnitYottabecquerels>;
pub type UnitZettabecquerels = DimensionStruct<P21, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Zettabecquerels<T> = Quantity<T, UnitZettabecquerels>;
pub type UnitExabecquerels = DimensionStruct<P18, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Exabecquerels<T> = Quantity<T, UnitExabecquerels>;
pub type UnitPetabecquerels = DimensionStruct<P15, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Petabecquerels<T> = Quantity<T, UnitPetabecquerels>;
pub type UnitTerabecquerels = DimensionStruct<P12, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Terabecquerels<T> = Quantity<T, UnitTerabecquerels>;
pub type UnitGigabecquerels = DimensionStruct<P9, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Gigabecquerels<T> = Quantity<T, UnitGigabecquerels>;
pub type UnitMegabecquerels = DimensionStruct<P6, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Megabecquerels<T> = Quantity<T, UnitMegabecquerels>;
pub type UnitKilobecquerels = DimensionStruct<P3, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Kilobecquerels<T> = Quantity<T, UnitKilobecquerels>;
pub type UnitHectobecquerels = DimensionStruct<P2, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Hectobecquerels<T> = Quantity<T, UnitHectobecquerels>;
pub type UnitDecabecquerels = DimensionStruct<P1, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Decabecquerels<T> = Quantity<T, UnitDecabecquerels>;
pub type UnitDecibecquerels = DimensionStruct<N1, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Decibecquerels<T> = Quantity<T, UnitDecibecquerels>;
pub type UnitCentibecquerels = DimensionStruct<N2, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Centibecquerels<T> = Quantity<T, UnitCentibecquerels>;
pub type UnitMillibecquerels = DimensionStruct<N3, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Millibecquerels<T> = Quantity<T, UnitMillibecquerels>;
pub type UnitMicrobecquerels = DimensionStruct<N6, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Microbecquerels<T> = Quantity<T, UnitMicrobecquerels>;
pub type UnitNanobecquerels = DimensionStruct<N9, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Nanobecquerels<T> = Quantity<T, UnitNanobecquerels>;
pub type UnitPicobecquerels = DimensionStruct<N12, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Picobecquerels<T> = Quantity<T, UnitPicobecquerels>;
pub type UnitFemtobecquerels = DimensionStruct<N15, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Femtobecquerels<T> = Quantity<T, UnitFemtobecquerels>;
pub type UnitAttobecquerels = DimensionStruct<N18, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Attobecquerels<T> = Quantity<T, UnitAttobecquerels>;
pub type UnitZeptobecquerels = DimensionStruct<N21, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Zeptobecquerels<T> = Quantity<T, UnitZeptobecquerels>;
pub type UnitYoctobecquerels = DimensionStruct<N24, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Yoctobecquerels<T> = Quantity<T, UnitYoctobecquerels>;
pub type UnitRontobecquerels = DimensionStruct<N27, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Rontobecquerels<T> = Quantity<T, UnitRontobecquerels>;
pub type UnitQuectobecquerels = DimensionStruct<N30, N1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Quectobecquerels<T> = Quantity<T, UnitQuectobecquerels>;