//! Macros for `rpc-rs`.

use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TTokenStream;
use quote::{quote, ToTokens};

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let input = TTokenStream::from(input);
    let hash = TTokenStream::from_str("#").unwrap();

    quote! {
        #hash [derive(serde::Serialize, serde::Deserialize)]
        #input
    }
    .to_token_stream()
    .into()
}
