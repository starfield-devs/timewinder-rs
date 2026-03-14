use std::{
    fmt::{Debug, Formatter, Result},
    marker::PhantomData,
};

use crate::traits::*;

#[derive(Default, PartialEq, Eq)]
pub struct SlicesContainer<'a> {
    pub slices: Vec<&'a str>,
}

impl<'a> SlicesContainer<'a> {
    pub fn new(slices: Vec<&'a str>) -> Self {
        Self { slices }
    }
}

#[derive(PartialEq, Eq)]
pub struct SlicesWrapper<'a, T> {
    pub inner: SlicesContainer<'a>,
    _marker: PhantomData<T>,
}

impl<'a, T> SlicesWrapper<'a, T> {
    pub fn new(slices: Vec<&'a str>) -> Self {
        Self {
            inner: SlicesContainer { slices },
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Default for SlicesWrapper<'a, T> {
    fn default() -> Self {
        Self {
            inner: SlicesContainer::default(),
            _marker: PhantomData,
        }
    }
}

impl<'a, T> Debug for SlicesWrapper<'a, T> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> Result {
        for s in &self.inner.slices {
            write!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl<'a, T> AsRef<[&'a str]> for SlicesWrapper<'a, T> {
    fn as_ref(&self) -> &[&'a str] {
        &self.inner.slices
    }
}

impl<'a, T> SlicesOps<'a> for SlicesWrapper<'a, T> {
    fn slices(&self) -> &[&'a str] {
        &self.inner.slices
    }
}

impl<'a, T> EqualsTarget<'a> for &SlicesWrapper<'a, T> {
    fn eq_impl(
        &self,
        slices: &[&'a str],
    ) -> bool {
        iter_chars(slices).eq(iter_chars(self.slices()))
    }
}
