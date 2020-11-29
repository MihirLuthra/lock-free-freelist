use super::{
    dump::Dump,
    reuse::Reuse,
    smart_pointer::SmartPointer,
    reusable::Reusable,
};
use std::ops::Deref;

/// A dump for throwing and reusing heap
/// allocated memory. Maximum entries it 
/// can store is equal to the number of bits in usize.
///
/// # Example
///
/// ```
/// use lock_free_freelist::{FreeList, Reusable};
///
/// #[derive(Reusable)]
/// struct MyType {
///     x: i32,
/// }
///
/// // A free list to store heap allocated pointers to i32
/// let free_list = FreeList::<Box<MyType>>::new();
///
/// // If free list contains free pointers, it will
/// // reuse that, otherwise will allocate new memory
/// let x = free_list.reuse_or_alloc(MyType { x: 5 });
/// ```
pub struct FreeList<T: SmartPointer>
where
    <T as Deref>::Target: Sized + Reusable,
{
    pub(crate) dump: Dump<<T as Deref>::Target>,
}

/// Calls self.clear()
impl<T: SmartPointer> Drop for FreeList<T>
where
    <T as Deref>::Target: Sized + Reusable,
{
    fn drop(&mut self) {
        unsafe {
            self.clear();
        }
    }
}

impl<T: SmartPointer> FreeList<T>
where
    <T as Deref>::Target: Sized + Reusable,
{
    /// Initialize an empty free list.
    ///
    /// # Example
    /// ```
    /// use lock_free_freelist::{FreeList, Reusable};
    ///
    /// #[derive(Reusable)]
    /// struct MyType;
    /// 
    /// let free_list = FreeList::<Box<MyType>>::new();
    /// ```
    pub fn new() -> Self {
        FreeList { dump: Dump::new() }
    }

    /// Returns a [Reuse](crate::Reuse) on success.
    /// On failure, it returns the contents back indicating that free list
    /// is empty.
    ///
    /// # Example
    /// ```
    /// use lock_free_freelist::{FreeList, Reusable};
    ///
    /// #[derive(Debug, PartialEq, Eq, Reusable)]
    /// struct MyType {
    ///     x: i32,
    /// }
    ///
    /// fn main() {
    ///     let free_list = FreeList::<Box<MyType>>::new();
    ///
    ///     // free list is empty, should return Err(())
    ///     assert!(free_list.reuse(MyType {x: 3}).is_err());
    ///
    ///     {
    ///         // We drop a value so free list contains something.
    ///         let _to_drop = free_list.alloc(MyType {x: 20});
    ///     }
    ///
    ///     let mut my_type = free_list.reuse(MyType {x: 9}).unwrap();
    ///
    ///     assert_eq!(**my_type, MyType {x: 9});
    /// }
    /// ```
    pub fn reuse<'a>(&'a self, contents: <T as Deref>::Target) -> Result<Reuse<'a, T>, <T as Deref>::Target> {
        if let Ok(ptr) = self.dump.recycle() {
            let mut reused = unsafe { T::from_raw(ptr) };
            reused.set_new_val(contents);

            Ok(Reuse::new(reused, self))
        } else {
            Err(contents)
        }
    }

    pub fn reuse_or_alloc<'a>(&'a self, contents: <T as Deref>::Target) -> Reuse<'a, T> {
        if let Ok(ptr) = self.dump.recycle() {
            let mut reused = unsafe { T::from_raw(ptr) };
            reused.set_new_val(contents);
            Reuse::new(reused, self)
        } else {
            self.alloc(contents)
        }
    }

    /// Calls [Reuse::new](crate::Reuse::new) with this free list.
    ///
    /// # Example
    /// ```
    /// use lock_free_freelist::FreeList;
    ///
    /// let free_list = FreeList::<Box<i32>>::new();
    ///
    /// let x = free_list.alloc(5);
    /// ```
    pub fn alloc<'a>(&'a self, contents: <T as Deref>::Target) -> Reuse<'a, T> {
        let allocated = T::new(contents);
        Reuse::new(allocated, self)
    }

    /// Calls drop for all the pointers in free list
    /// and clears the free list.
    ///
    /// This is not thread safe.
    ///
    /// # Example
    /// ```
    /// use lock_free_freelist::FreeList;
    ///
    /// let free_list = FreeList::<Box<i32>>::new();
    ///
    /// unsafe{
    ///     free_list.clear();
    /// }
    /// ```
    pub unsafe fn clear(&self) {
        // drop all the pointers that are still on free list
        self.dump.for_each(|ptr| {
            let _ = T::from_raw(ptr);
        });
    }
}
