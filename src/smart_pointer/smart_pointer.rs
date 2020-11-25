use std::ops::Deref;

pub trait SmartPointer: Deref
where
    <Self as Deref>::Target: Sized,
{
    unsafe fn from_raw(raw: *mut <Self as Deref>::Target) -> Self;
    fn into_raw(smart_pointer: Self) -> *mut <Self as Deref>::Target;
}
