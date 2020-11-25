use super::{dump::Dump, fbox::FBox, smart_pointer::SmartPointer};

use std::mem::ManuallyDrop;

pub struct FreeList<T: SmartPointer> {
    pub(crate) dump: Dump<T::Content>,
}

impl<T: SmartPointer> FreeList<T> {
    pub fn new() -> Self {
        FreeList { dump: Dump::new() }
    }

    pub fn recycle<'a>(&'a self) -> Result<FBox<'a, T>, ()> {
        if let Ok(ptr) = self.dump.recycle() {
            return Ok(FBox {
                smart_pointer: unsafe { ManuallyDrop::new(T::from_raw(ptr)) },
                free_list: self,
            });
        }

        Err(())
    }
}
