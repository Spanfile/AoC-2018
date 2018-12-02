extern crate proc_macro;

mod input;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let day = attr.to_string();
    let mut new_stream = TokenStream::new();
    // new_stream.extend("".parse::<TokenStream>());
    // new_stream.extend(item);

    for tree in item.into_iter() {
        match tree {
            TokenTree::Group => println!("Group: {}", tree);
        }
    }

    new_stream
}
