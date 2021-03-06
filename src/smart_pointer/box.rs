use super::smart_pointer::SmartPointer;
use crate::Reusable;
use std::ops::Deref;

unsafe impl<T> SmartPointer for Box<T>
where
    <Self as Deref>::Target: Sized + Reusable,
{
    unsafe fn from_raw(raw: *mut <Self as Deref>::Target) -> Self {
        Box::from_raw(raw)
    }

    fn into_raw(smart_pointer: Self) -> *mut <Self as Deref>::Target {
        Box::into_raw(smart_pointer)
    }

    fn new(contents: <Self as Deref>::Target) -> Self {
        Box::new(contents)
    }
}
