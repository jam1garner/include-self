#![feature(proc_macro_span)]

use proc_macro::TokenStream;
use std::path::Path;

#[proc_macro]
pub fn include_self(x: TokenStream) -> TokenStream {
    let file = match x.into_iter().nth(0) {
        Some(x) => Path::new("..").join(x.span().source_file().path()).to_str().unwrap().to_owned(),
        None => panic!("Good vibes required. please pass some vibes in")
    };
    quote::quote!(
        include!(#file)
    ).into()
}
