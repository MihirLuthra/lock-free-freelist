use std::ops::Deref;

/// Types implementing this trait can be wrapped inside
/// [FreeList](crate::FreeList).
///
/// This trait requires the user to guarantee that
/// the type implementing SmartPointer will _not_ use
/// the pointer returned from `into_raw()` after being is dropped.
///
/// So, this trait should _not_ be implemented for [Arc](std::sync::Arc), [Rc](std::rc::Rc) etc.
/// becuase the pointer could still be out there after being dropped.
///
/// For this reason the trait is unsafe.
pub unsafe trait SmartPointer: Deref
where
    <Self as Deref>::Target: Sized,
{
    /// Constructs an instance of Self by a raw pointer.
    unsafe fn from_raw(raw: *mut <Self as Deref>::Target) -> Self;

    /// Consumes Self to return the contained raw pointer.
    /// This trait assumes if it extracts the raw pointer using `into_raw()`,
    /// it won't be changed from anywhere else.
    fn into_raw(smart_pointer: Self) -> *mut <Self as Deref>::Target;
}
