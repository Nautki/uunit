use crate::*;
pub type UnitMicron = DimensionStruct<N6, Z0, P1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type Micron<T> = Quantity<T, UnitMicron>;