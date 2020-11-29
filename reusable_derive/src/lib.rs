extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use quote::quote;

#[proc_macro_derive(Reusable)]
pub fn reusable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics lock_free_freelist::Reusable for  #name #ty_generics #where_clause {
            fn set_new_val(&mut self, other: Self) {
                let _old_val = std::mem::replace(self, other);
            }
        }
    };

    TokenStream::from(expanded)
}
