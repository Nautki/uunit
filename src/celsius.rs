use crate::*;
pub type UnitCelsius = DimensionStruct<P1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, P1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Celsius<T> = Quantity<T, UnitCelsius>;