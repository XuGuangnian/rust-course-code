#![allow(dead_code, unused_variables)]

use proc_macro::TokenStream;

use quote::quote;

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // todo
    let gen = quote! {};
    gen.into()
}
