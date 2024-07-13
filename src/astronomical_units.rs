use crate::*;
pub type UnitAstronomicalUnits = DimensionStruct<P1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, P1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type AstronomicalUnits<T> = Quantity<T, UnitAstronomicalUnits>;