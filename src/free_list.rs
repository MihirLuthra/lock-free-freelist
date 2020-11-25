use super::{
    dump::Dump,
    fbox::FBox,
    smart_pointer::SmartPointer,
};

use std::mem::ManuallyDrop;

pub struct FreeList<T: SmartPointer> {
    pub(crate) dump: Dump<T::Content>,
}

impl<T: SmartPointer> Drop for FreeList<T> {
    fn drop(&mut self) {
        // drop all the pointers that are still on free list
        self.dump.for_each(|ptr| {
            let _ = unsafe { T::from_raw(ptr) };
        });
    }
}

impl<T: SmartPointer> FreeList<T> {
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
}
