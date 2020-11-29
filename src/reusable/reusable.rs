/// This trait is required by [Deref Target](std::ops::Deref::Target)
/// of a type implementing [SmartPointer](crate::SmartPointer).
///
/// When a free list reuses an element, the old contents are still there.
/// The definition of [set_new_val](crate::Reusable::set_new_val)
/// should use the new instance of type `Self` to set it to new contents.
///
/// [`#[derive(Reusable)]`](reusable_derive::Reusable) defines
/// `set_new_val` to just perform a [std::mem::replace] to set it to new contents
pub trait Reusable {
    /// Contents of `other` should be assigned to
    /// `self` in whatever way the implementer finds efficient.
    fn set_new_val(&mut self, other: Self);
}

macro_rules! impl_reusable {
    ($($ty: ty),*) => {
        $(
            impl Reusable for $ty {
                fn set_new_val(&mut self, other: Self) {
                    let _old_val = std::mem::replace(self, other);
                }
            }
        )*
    };
}

macro_rules! impl_generic_reusable {
    ($ty: ty, $( $impl_gen: tt ),*) => {
        impl<$($impl_gen),*> Reusable for $ty {
            fn set_new_val(&mut self, other: Self) {
                let _old_val = std::mem::replace(self, other);
            }
        }
    };
}

impl_reusable!(u8, i8, u16, i16, i32, u32, i64, u64, i128, u128);
impl_reusable!(String);
impl_generic_reusable!(Option<T>, T);
impl_generic_reusable!(Result<T, E>, T, E);
