use crate::*;
pub type UnitMetricTon = DimensionStruct<P6, Z0, Z0, P1, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0, Z0>;
pub type MetricTon<T> = Quantity<T, UnitMetricTon>;