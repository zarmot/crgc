use std::ops::Deref;
use std::ptr::NonNull;

use super::{append, remove, temp, Trace, Tracer};

pub(crate) struct GcValue<T: Trace> {
    count: usize,
    index: usize,
    value: T,
}
impl<T: Trace> GcValue<T> {
    #[inline]
    unsafe fn new(value: T) -> NonNull<GcValue<T>> {
        let b = Box::new(GcValue {
            count: 0,
            index: 0,
            value,
        });
        let mut ptr = NonNull::new(Box::into_raw(b)).unwrap();
        ptr.as_mut().index = append(ptr);
        return ptr;
    }
    #[inline]
    unsafe fn inc(&mut self) {
        self.count += 1;
    }
    #[inline]
    unsafe fn dec(&mut self) {
        self.count -= 1;
        if self.count == 0 {
            remove(self.index);
            self.del();
        }
    }
    #[inline]
    unsafe fn del(&mut self) {
        drop(Box::from_raw(self));
    }
}
impl<T: Trace> Tracer for GcValue<T> {
    #[inline]
    fn count(&self) -> usize {
        self.count
    }
    #[inline]
    fn index(&self) -> usize {
        self.index
    }
    #[inline]
    fn run_step1(&mut self) {
        self.value.step1();
    }
    #[inline]
    fn run_step2(&mut self) {
        self.value.step2();
    }
    #[inline]
    unsafe fn free(&mut self) {
        self.del();
    }
}

pub struct Gc<T: Trace> {
    pub(crate) raw: NonNull<GcValue<T>>,
}
impl<T: Trace> Gc<T> {
    #[inline]
    pub fn new(value: T) -> Self {
        unsafe {
            Gc {
                raw: GcValue::new(value),
            }
        }
    }
}
impl<T: Trace> Clone for Gc<T> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            (*self.raw.as_ptr()).inc();
            return Gc { raw: self.raw };
        }
    }
}
impl<T: Trace> Drop for Gc<T> {
    fn drop(&mut self) {
        unsafe {
            (*self.raw.as_ptr()).dec();
        }
    }
}

impl<T: Trace> Deref for Gc<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.raw.as_ptr()).value }
    }
}

impl<T: Trace> Trace for Gc<T> {
    #[inline]
    fn step1(&self) {
        unsafe {
            (*self.raw.as_ptr()).count -= 1;
            if (*self.raw.as_ptr()).count == 0 {
                temp((*self.raw.as_ptr()).index);
            }
        }
    }
    #[inline]
    fn step2(&self) {
        unsafe {
            (*self.raw.as_ptr()).count += 1;
        }
    }
}
