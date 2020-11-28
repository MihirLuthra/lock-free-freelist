use super::{free_list::FreeList, smart_pointer::SmartPointer};
use std::{mem::ManuallyDrop, ops::{Deref, DerefMut}};

/// This is a wrapper around smart pointers so that
/// when they are dropped, raw pointers contained in them can
/// be put to free list and reused.
///
/// This can be produced by [FreeList::alloc](crate::FreeList::alloc)
/// or [FreeList::recycle](crate::FreeList::recycle).
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
/// let reusable_box: Reuse<Box<i32>> = free_list.alloc(Box::new(5));
///
/// drop(reusable_box); // So that it can be recycled
///
/// let mut new_reusable_box: Reuse<Box<i32>> = free_list.recycle().unwrap();
/// **new_reusable_box = 5;
/// ```
pub struct Reuse<'a, T: SmartPointer>
where
    <T as Deref>::Target: Sized,
{
    smart_pointer: ManuallyDrop<T>,
    free_list: &'a FreeList<T>,
}

impl<'a, T: SmartPointer> Reuse<'a, T>
where
    <T as Deref>::Target: Sized,
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
    <T as Deref>::Target: Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.smart_pointer
    }
}

impl<'a, T: SmartPointer> DerefMut for Reuse<'a, T>
where
    <T as Deref>::Target: Sized,
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
    <T as Deref>::Target: Sized,
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
