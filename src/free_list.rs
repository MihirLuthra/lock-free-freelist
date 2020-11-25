use super::{
    dump::Dump,
    fbox::FBox,
    smart_pointer::{InitializableSmartPointer, SmartPointer},
};

use std::mem::ManuallyDrop;

pub struct FreeList<T: SmartPointer> {
    pub(crate) dump: Dump<T::Content>,
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

impl<T: InitializableSmartPointer> FreeList<T> {
    pub fn recycle_or_alloc<'a>(
        &'a self,
        alloc_contents: <T as SmartPointer>::Content,
    ) -> FBox<'a, T>
    where
        T: InitializableSmartPointer,
    {
        if let Ok(fbox) = self.recycle() {
            fbox
        } else {
            FBox {
                smart_pointer: ManuallyDrop::new(T::new(alloc_contents)),
                free_list: self,
            }
        }
    }
}
