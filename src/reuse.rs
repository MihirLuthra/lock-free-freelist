use super::{
    free_list::FreeList,
    smart_pointer::SmartPointer,
    reusable::Reusable,
};
use std::{mem::ManuallyDrop, ops::{Deref, DerefMut}};

/// This is a wrapper around smart pointers so that
/// when they are dropped, raw pointers contained in them can
/// be put to free list and reused.
///
/// This is produced by a [FreeList](crate::FreeList).
/// It is a smart pointer that contains shared ref to FreeList
/// and when it's contents drop, the contained pointer of those contents
/// is dropped into free list for later use.
///
/// It implements Deref and DerefMut to access the wrapped smart pointer.
///
/// # Example
///
/// ```
/// use lock_free_freelist::{FreeList, Reuse};
///
/// let free_list = FreeList::<Box<i32>>::new();
///
/// let reusable_box: Reuse<Box<i32>> = free_list.alloc(5);
///
/// drop(reusable_box); // So that it can be reused
///
/// let mut new_reusable_box: Reuse<Box<i32>> = free_list.reuse(9).unwrap();
///
/// assert_eq!(**new_reusable_box, 9);
/// ```
pub struct Reuse<'a, T: SmartPointer>
where
    <T as Deref>::Target: Sized + Reusable,
{
    smart_pointer: ManuallyDrop<T>,
    free_list: &'a FreeList<T>,
}

impl<'a, T: SmartPointer> Reuse<'a, T>
where
    <T as Deref>::Target: Sized + Reusable,
{
    /// Get a new [Reuse](crate::Reuse) instance.
    pub fn new<'b>(smart_pointer: T, free_list: &'b FreeList<T>) -> Reuse<'b, T> {
        Reuse {
            smart_pointer: ManuallyDrop::new(smart_pointer),
            free_list: free_list,
        }
    }
}

impl<'a, T: SmartPointer> Deref for Reuse<'a, T>
where
    <T as Deref>::Target: Sized + Reusable,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.smart_pointer
    }
}

impl<'a, T: SmartPointer> DerefMut for Reuse<'a, T>
where
    <T as Deref>::Target: Sized + Reusable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.smart_pointer
    }
}

/// When the instance of this type is dropped,
/// an attempt is made to put the pointer to the contenst into free list
/// and if free list is full, the contents are dropped.
impl<'a, T: SmartPointer> Drop for Reuse<'a, T>
where
    <T as Deref>::Target: Sized + Reusable,
{
    fn drop(&mut self) {
        let smart_pointer = unsafe { ManuallyDrop::take(&mut self.smart_pointer) };

        let garbage = T::into_raw(smart_pointer);

        // Try to add this memory to free list and if free list
        // is full then drop it.
        if let Err(ptr) = self.free_list.dump.throw(garbage) {
            // We come here if the dump is full.
            // Here we will have to drop the value instead
            // of storing it in dump.
            unsafe {
                let _to_drop = T::from_raw(ptr);
            }
        }
    }
}
