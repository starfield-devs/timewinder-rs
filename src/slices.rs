use std::{
    fmt::{Debug, Formatter, Result},
    marker::PhantomData,
};

use crate::traits::*;

#[derive(PartialEq, Eq)]
pub struct Slices<'a, T> {
    pub slices: Vec<&'a str>,
    _marker: PhantomData<T>,
}

impl<'a, T> Slices<'a, T> {
    pub fn new(slices: Vec<&'a str>) -> Self {
        Self {
            slices,
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Default for Slices<'a, T> {
    fn default() -> Self {
        Self {
            slices: Vec::default(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Debug for Slices<'a, T> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result {
        for s in &self.slices {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl<'a, T> AsRef<[&'a str]> for Slices<'a, T> {
    fn as_ref(&self) -> &[&'a str] {
        &self.slices
    }
}

impl<'a, T> SlicesOps<'a> for Slices<'a, T> {
    fn slices(&self) -> &[&'a str] {
        &self.slices
    }
}

impl<'a, T> EqualsTarget<'a> for &Slices<'a, T> {
    fn eq_impl(
        &self,
        slices: &[&'a str],
    ) -> bool {
        iter_chars(slices).eq(iter_chars(self.slices()))
    }
}
