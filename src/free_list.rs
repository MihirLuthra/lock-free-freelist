use super::{dump::Dump, fbox::FBox, smart_pointer::SmartPointer};
use std::ops::Deref;

pub struct FreeList<T: SmartPointer>
where
    <T as Deref>::Target: Sized,
{
    pub(crate) dump: Dump<<T as Deref>::Target>,
}

/// Calls self.clear()
impl<T: SmartPointer> Drop for FreeList<T>
where
    <T as Deref>::Target: Sized,
{
    fn drop(&mut self) {
        unsafe {
            self.clear();
        }
    }
}

impl<T: SmartPointer> FreeList<T>
where
    <T as Deref>::Target: Sized,
{
    /// Initialize an empty free list.
    ///
    /// # Example
    /// ```
    /// use lock_free_freelist::FreeList;
    ///
    /// struct MyType;
    /// 
    /// let free_list = FreeList::<Box<MyType>>::new();
    /// ```
    pub fn new() -> Self {
        FreeList { dump: Dump::new() }
    }

    /// Returns a FBox on success.
    /// On failure, it returns () indicating that free list
    /// is empty.
    ///
    /// You have to setup the contents of the recycled value by yourself.
    ///
    /// # Example
    /// ```
    /// use lock_free_freelist::{FreeList, FBox};
    ///
    /// #[derive(Debug, PartialEq, Eq)]
    /// struct MyType {
    ///     x: i32,
    /// }
    ///
    /// fn main() {
    ///     let free_list = FreeList::<Box<MyType>>::new();
    ///
    ///     // free list is empty, should return Err(())
    ///     assert!(free_list.recycle().is_err());
    ///
    ///     {
    ///         // We drop a value so free list contains something.
    ///         let my_type = Box::new( MyType {x: 5} );
    ///         let _to_drop = FBox::new(my_type, &free_list);
    ///     }
    ///
    ///     let mut my_type = free_list.recycle().unwrap();
    ///
    ///     // Needs to be set explicitly
    ///     my_type.x = 9;
    ///
    ///     assert_eq!(**my_type, MyType {x: 9});
    /// }
    /// ```
    pub fn recycle<'a>(&'a self) -> Result<FBox<'a, T>, ()> {
        let ptr = self.dump.recycle()?;

        Ok(FBox::new(
            unsafe { T::from_raw(ptr) },
            self,
        ))
    }

    pub fn alloc<'a>(&'a self, smart_pointer: T) -> FBox<'a, T> {
        FBox::new(smart_pointer, self)
    }

    /// Calls drop for all the pointers in free list
    /// and clear the free list.
    ///
    /// This is not thread safe.
    pub unsafe fn clear(&self) {
        // drop all the pointers that are still on free list
        self.dump.for_each(|ptr| {
            let _ = T::from_raw(ptr);
        });
    }
}
