use std::cell::{Cell, RefCell};

use super::Trace;

impl Trace for () {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for i8 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for i16 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for i32 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for i64 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for i128 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for isize {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for u8 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for u16 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for u32 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for u64 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for u128 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for usize {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for f32 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for f64 {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for char {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
impl Trace for bool {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}

impl<T: Trace, const N: usize> Trace for [T; N] {
    #[inline]
    fn step1(&self) {
        for item in self.iter() {
            item.step1();
        }
    }
    #[inline]
    fn step2(&self) {
        for item in self.iter() {
            item.step2();
        }
    }
}

impl<T: Trace> Trace for Option<T> {
    #[inline]
    fn step1(&self) {
        if let Some(v) = self {
            v.step1();
        }
    }
    #[inline]
    fn step2(&self) {
        if let Some(v) = self {
            v.step2();
        }
    }
}

impl<T: Trace> Trace for RefCell<T> {
    #[inline]
    fn step1(&self) {
        self.borrow().step1();
    }
    #[inline]
    fn step2(&self) {
        self.borrow().step2();
    }
}
impl<T: Trace + Copy> Trace for Cell<T> {
    #[inline]
    fn step1(&self) {}
    #[inline]
    fn step2(&self) {}
}
