pub trait Reusable {
    fn set_new_val(&mut self, other: Self);
}

#[macro_export]
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

#[macro_export]
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
