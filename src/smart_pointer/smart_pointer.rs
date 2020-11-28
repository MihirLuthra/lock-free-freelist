use std::ops::Deref;

/// This trait requires the user to guarantee that
/// the type implementing SmartPointer will not use the
/// the pointer returned from `into_raw()` after it is dropped.
///
/// So, this trait should _not_ be implemented for Arc, Rc etc.
/// becuase the pointer could still be out there after one instance
/// of Arc is dropped.
///
/// For this reason the trait is unsafe.
pub unsafe trait SmartPointer: Deref
where
    <Self as Deref>::Target: Sized,
{
    unsafe fn from_raw(raw: *mut <Self as Deref>::Target) -> Self;
    fn into_raw(smart_pointer: Self) -> *mut <Self as Deref>::Target;
}
