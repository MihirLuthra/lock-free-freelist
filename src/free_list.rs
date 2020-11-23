use super::{
    fbox::FBox,
    dump::Dump,
    smart_pointer::SmartPointer,
};

use std::marker::PhantomData;

pub struct FreeList<'a, T: SmartPointer> {
    dump: Dump<T>,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: SmartPointer> FreeList<'a, T> {
    pub fn new() -> Self {
        FreeList { 
            dump: Dump::new(),
            _marker: PhantomData,
        }
    }

    pub fn alloc() -> FBox<'a, T> {
       todo!() 
    }
}
