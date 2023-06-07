use std::ops::{Deref, DerefMut};

use super::Trace;

pub struct Val<T> {
    pub value: T,
}
impl<T: 'static> Trace for Val<T> {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}

impl<T> Val<T> {
    #[inline]
    pub fn new(value: T) -> Val<T> {
        Val { value }
    }
}
impl<T> From<T> for Val<T> {
    #[inline]
    fn from(value: T) -> Self {
        Val { value }
    }
}

impl<T> Deref for Val<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T> DerefMut for Val<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
