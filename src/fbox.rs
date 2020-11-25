use super::{free_list::FreeList, smart_pointer::SmartPointer};
use std::{mem::ManuallyDrop, ops::Deref};

pub struct FBox<'a, T: SmartPointer> {
    pub(crate) smart_pointer: ManuallyDrop<T>,
    pub(crate) free_list: &'a FreeList<T>,
}

impl<'a, T: SmartPointer> Deref for FBox<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.smart_pointer
    }
}

impl<'a, T: SmartPointer> Drop for FBox<'a, T> {
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
