use super::{dump::Dump, fbox::FBox, smart_pointer::SmartPointer};
use std::{mem::ManuallyDrop, ops::Deref};

pub struct FreeList<T: SmartPointer>
where
    <T as Deref>::Target: Sized,
{
    pub(crate) dump: Dump<<T as Deref>::Target>,
}

impl<T: SmartPointer> Drop for FreeList<T>
where
    <T as Deref>::Target: Sized,
{
    fn drop(&mut self) {
        // drop all the pointers that are still on free list
        self.dump.for_each(|ptr| {
            let _ = unsafe { T::from_raw(ptr) };
        });
    }
}

impl<T: SmartPointer> FreeList<T>
where
    <T as Deref>::Target: Sized,
{
    pub fn new() -> Self {
        FreeList { dump: Dump::new() }
    }

    pub fn recycle<'a>(&'a self) -> Result<FBox<'a, T>, ()> {
        let ptr = self.dump.recycle()?;

        Ok(FBox {
            smart_pointer: unsafe { ManuallyDrop::new(T::from_raw(ptr)) },
            free_list: self,
        })
    }

    pub fn alloc<'a>(&'a self, smart_pointer: T) -> FBox<'a, T> {
        FBox {
            smart_pointer: ManuallyDrop::new(smart_pointer),
            free_list: self,
        }
    }
}
