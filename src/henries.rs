use crate::*;
pub type UnitHenries = DimensionStruct<P3, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Henries<T> = Quantity<T, UnitHenries>;
pub type UnitQuettahenries = DimensionStruct<P33, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Quettahenries<T> = Quantity<T, UnitQuettahenries>;
pub type UnitRonnahenries = DimensionStruct<P30, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Ronnahenries<T> = Quantity<T, UnitRonnahenries>;
pub type UnitYottahenries = DimensionStruct<P27, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Yottahenries<T> = Quantity<T, UnitYottahenries>;
pub type UnitZettahenries = DimensionStruct<P24, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Zettahenries<T> = Quantity<T, UnitZettahenries>;
pub type UnitExahenries = DimensionStruct<P21, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Exahenries<T> = Quantity<T, UnitExahenries>;
pub type UnitPetahenries = DimensionStruct<P18, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Petahenries<T> = Quantity<T, UnitPetahenries>;
pub type UnitTerahenries = DimensionStruct<P15, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Terahenries<T> = Quantity<T, UnitTerahenries>;
pub type UnitGigahenries = DimensionStruct<P12, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Gigahenries<T> = Quantity<T, UnitGigahenries>;
pub type UnitMegahenries = DimensionStruct<P9, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Megahenries<T> = Quantity<T, UnitMegahenries>;
pub type UnitKilohenries = DimensionStruct<P6, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Kilohenries<T> = Quantity<T, UnitKilohenries>;
pub type UnitHectohenries = DimensionStruct<P5, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Hectohenries<T> = Quantity<T, UnitHectohenries>;
pub type UnitDecahenries = DimensionStruct<P4, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Decahenries<T> = Quantity<T, UnitDecahenries>;
pub type UnitDecihenries = DimensionStruct<P2, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Decihenries<T> = Quantity<T, UnitDecihenries>;
pub type UnitCentihenries = DimensionStruct<P1, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Centihenries<T> = Quantity<T, UnitCentihenries>;
pub type UnitMillihenries = DimensionStruct<Z0, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Millihenries<T> = Quantity<T, UnitMillihenries>;
pub type UnitMicrohenries = DimensionStruct<N3, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Microhenries<T> = Quantity<T, UnitMicrohenries>;
pub type UnitNanohenries = DimensionStruct<N6, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Nanohenries<T> = Quantity<T, UnitNanohenries>;
pub type UnitPicohenries = DimensionStruct<N9, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Picohenries<T> = Quantity<T, UnitPicohenries>;
pub type UnitFemtohenries = DimensionStruct<N12, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Femtohenries<T> = Quantity<T, UnitFemtohenries>;
pub type UnitAttohenries = DimensionStruct<N15, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Attohenries<T> = Quantity<T, UnitAttohenries>;
pub type UnitZeptohenries = DimensionStruct<N18, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Zeptohenries<T> = Quantity<T, UnitZeptohenries>;
pub type UnitYoctohenries = DimensionStruct<N21, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Yoctohenries<T> = Quantity<T, UnitYoctohenries>;
pub type UnitRontohenries = DimensionStruct<N24, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Rontohenries<T> = Quantity<T, UnitRontohenries>;
pub type UnitQuectohenries = DimensionStruct<N27, N2, P2, P1, N2, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Quectohenries<T> = Quantity<T, UnitQuectohenries>;