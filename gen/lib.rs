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