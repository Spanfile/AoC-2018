extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

use proc_macro2::{Ident, TokenStream, Span};

#[proc_macro_attribute]
pub fn aoc(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr: TokenStream = attr.into();
    let item: TokenStream = item.into();

    let mut new_stream = TokenStream::new();
    new_stream.extend(item.clone());

    let mut iter = item.into_iter();
    iter.next();
    match iter.next() {
        None => panic!("error: no ident"),
        Some(ident) => {
            let new_ident = Ident::new(&format!("do_{}", ident.to_string()), Span::call_site());
            let replacement = quote! {
                pub fn #new_ident() {
                    #ident(input::get(#attr));
                }
            };
            new_stream.extend(replacement);
        }
    };

    new_stream.into()
}
