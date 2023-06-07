use std::cell::RefCell;
use std::ptr::NonNull;

thread_local! {
    static CRGC: RefCell<CycleRemover> = RefCell::new(CycleRemover::init());
    static TEMP: RefCell<Vec<usize>> = RefCell::new(Vec::new());
}

pub trait Trace: 'static {
    fn step1(&self);
    fn step2(&self);
}
pub(crate) trait Tracer {
    fn count(&self) -> usize;
    fn index(&self) -> usize;
    fn run_step1(&mut self);
    fn run_step2(&mut self);
    unsafe fn free(&mut self);
}

pub struct CycleRemover {
    pub(crate) list: Vec<Option<NonNull<dyn Tracer>>>,
    pub(crate) stack: Vec<usize>,
}
impl CycleRemover {
    #[inline]
    fn init() -> CycleRemover {
        CycleRemover {
            list: Vec::new(),
            stack: Vec::new(),
        }
    }
}
impl CycleRemover {
    #[inline]
    fn append(&mut self, meta: NonNull<dyn Tracer>) -> usize {
        if let Some(i) = self.stack.pop() {
            self.list[i] = Some(meta);
            return i;
        } else {
            let i = self.list.len();
            self.list.push(Some(meta));
            return i;
        }
    }
    #[inline]
    fn remove(&mut self, i: usize) {
        self.list[i] = None;
        self.stack.push(i);
    }
}
impl CycleRemover {
    #[inline]
    fn temp(i: usize) {
        TEMP.with(|temp| temp.borrow_mut().push(i));
    }
    #[inline]
    fn collect(&mut self) {
        let iter = self.list.iter();
        for i in iter {
            if let Some(a) = i {
                unsafe {
                    (*(*a).as_ptr()).run_step1();
                }
            }
        }
        TEMP.with(|tref| {
            let mut temp = tref.borrow_mut();
            temp.sort();

            let mut titer = temp.iter();
            let mut ti = titer.next();

            let mut ci = 0;
            let iter = self.list.iter();
            for i in iter {
                let step2 = || {
                    if let Some(a) = i {
                        unsafe {
                            (*(*a).as_ptr()).run_step2();
                        }
                    }
                };
                if let Some(x) = ti {
                    if *x != ci {
                        step2();
                    } else {
                        ti = titer.next();
                    }
                } else {
                    step2();
                }
                ci += 1;
            }

            let iter = temp.iter();
            for i in iter {
                if let Some(x) = self.list[*i] {
                    unsafe {
                        if (*x.as_ptr()).count() == 0 {
                            self.remove((*x.as_ptr()).index());
                            (*x.as_ptr()).free();
                        }
                    }
                }
            }
            temp.clear();
        });
    }
}

pub fn collect() {
    CRGC.with(|crgc| crgc.borrow_mut().collect())
}
pub(crate) fn append(meta: NonNull<dyn Tracer>) -> usize {
    CRGC.with(|crgc| crgc.borrow_mut().append(meta))
}
pub(crate) fn remove(i: usize) {
    CRGC.with(|crgc| crgc.borrow_mut().remove(i));
}
pub(crate) fn temp(i: usize) {
    CycleRemover::temp(i);
}
