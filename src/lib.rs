#![no_std]

use core::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};
use paste::paste;

#[derive(Debug, Clone, Copy)]
pub struct Quantity<T, U> {
    pub value: T,
    pub unit: U,
}

impl<T, U> Quantity<T, U> {
    pub fn new(value: T, unit: U) -> Self {
        Quantity { value, unit }
    }
}

macro_rules! impl_prims {
    ($($ty:ty),*) => {
        $(
            impl <U> From<Quantity<$ty, U>> for $ty {
                fn from(value: Quantity<$ty, U>) -> Self {
                    value.value
                }
            }
        )*
    };
}

impl_prims! {
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    f32, f64
}

macro_rules! impl_ops {
    ($($trait:ident $fn:ident, $trait_assign:ident $fn_assign:ident);*) => {
        $(
            impl <T: $trait<T>, U: $trait<U>> $trait<Quantity<T, U>> for Quantity<T, U> {
                type Output = Quantity<<T as $trait<T>>::Output, <U as $trait<U>>::Output>;

                fn $fn(self, rhs: Quantity<T, U>) -> Self::Output {
                    Quantity::new(self.value.$fn(rhs.value), self.unit.$fn(rhs.unit))
                }
            }

            impl <T: $trait_assign<T>, U: $trait_assign<U>> $trait_assign<Quantity<T, U>> for Quantity<T, U> {
                fn $fn_assign(&mut self, rhs: Quantity<T, U>) {
                    self.value.$fn_assign(rhs.value);
                    self.unit.$fn_assign(rhs.unit);
                }
            }
        )*
    };
}

impl_ops! {
    Add add, AddAssign add_assign;
    Sub sub, SubAssign sub_assign;
    Mul mul, MulAssign mul_assign;
    Div div, DivAssign div_assign;
    Rem rem, RemAssign rem_assign
}

macro_rules! impl_unit_mul {
    ($unit:ident; $($prim:ty),*) => {
        paste! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct [< $unit:camel Mul >];
        }

        $(
            paste! {
                impl Mul<[< $unit:camel Mul >]> for $prim {
                    type Output = Quantity<$prim, $unit>;
        
                    fn mul(self, _: [< $unit:camel Mul >]) -> Self::Output {
                        Quantity::new(self, [< $unit:camel >])
                    }
                }
            }
            
        )*
    };
}

macro_rules! impl_units {
    ($($ty:ident $(= $($eq_ty:ty),*)?);*) => {
        //pub struct DerivedUnit<$($ty: Integer),*>;
        
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $ty;

            impl Default for $ty {
                fn default() -> Self {
                    $ty
                }
            }

            impl Add<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn add(self, _: $ty) -> Self::Output {
                    $ty
                }
            }

            impl AddAssign<$ty> for $ty {
                #[inline]
                fn add_assign(&mut self, _: $ty) {
                    // noop
                }
            }

            impl Sub<$ty> for $ty {
                type Output = $ty;

                #[inline]
                fn sub(self, _: $ty) -> Self::Output {
                    $ty
                }
            }

            impl SubAssign<$ty> for $ty {
                #[inline]
                fn sub_assign(&mut self, _: $ty) {
                    // noop
                }
            }

            impl_unit_mul! {
                $ty;
                i8, i16, i32, i64, i128, isize,
                u8, u16, u32, u64, u128, usize,
                f32, f64
            }

            $(
                $(
                    impl From<$eq_ty> for $ty {
                        fn from(_: $eq_ty) -> Self {
                            $ty
                        }
                    }

                    impl From<$ty> for $eq_ty {
                        fn from(_: $ty) -> Self {
                            Self::default()
                        }
                    }
                )*
            )?
        )*
    };
}

macro_rules! cute_alias {
    ($($unit:ident),*) => {
        $(
            paste! {
                pub type [< $unit:camel >]<T> = Quantity<T, [< Unit $unit:camel >]>;
                #[allow(non_upper_case_globals)]
                pub static [< $unit:camel >]: [< Unit $unit:camel Mul >] = [< Unit $unit:camel Mul >];
            }
        )*
        
    }
}

macro_rules! impl_si_unit {
    ($unit: ident; $($prefix:ident : $mul:literal),*) => {
        paste! {
            impl_units! {
                [< Unit $unit:camel >];
                $([< Unit $prefix:camel $unit >]);*
            }

            cute_alias! {
                $unit, $([< $prefix:camel $unit >]),*
            }
        }
    };
}

macro_rules! define_units {
    ($($unit:ident);* $(;)?) => {
        paste! {
            $(
                impl_si_unit! {
                    $unit;
                    quetta:  30,
                    ronna:   27,
                    yotta:   24,
                    zetta:   21,
                    exa:     18,
                    peta:    15,
                    tera:    12,
                    giga:     9,
                    mega:     6,
                    kilo:     3,
                    hecto:    2,
                    deca:     1,
                    deci:    -1,
                    centi:   -2,
                    milli:   -3,
                    micro:   -6,
                    nano:    -9,
                    pico:   -12,
                    femto:  -15,
                    atto:   -18,
                    zepto:  -21,
                    yocto:  -24,
                    ronto:  -27,
                    quecto: -30
                }
            )*
            
        }
    };
}

define_units! {
    // SI Base units
    seconds;
    meters;
    grams;
    amperes;
    kelvin;
    moles;
    candelas;

    // SI Derived
    radians;
    steradians;
    hertz;
    newtons;
    pascals;
    joules;
    watts;
    coulombs;
    volts;
    farads;
    ohms;
    siemens;
    webers;
    teslas;
    henries;
    celsius;
    lumens;
    lux;
    becquerels;
    grays;
    sieverts;
    katals;

    // Metric non-SI
    kaysers;
    gals;
    dynes;
    baryes;
    ergs;
    poises;
    stokes;
    stilbs;
    phots;
    rayls;

    // MKpS
    gramForce;
    hyls;
    poncelets;
}