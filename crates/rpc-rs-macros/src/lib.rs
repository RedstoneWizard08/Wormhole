//! Macros for `rpc-rs`.

use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TTokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(Event)]
pub fn derive_event(raw_input: TokenStream) -> TokenStream {
    let input = TTokenStream::from(raw_input.clone());
    let hash = TTokenStream::from_str("#").unwrap();
    let struct_ = parse_macro_input!(raw_input as ItemStruct);
    let name = struct_.ident;
    let name_str = format!("\"{}\"", name);

    quote! {
        #hash [derive(serde::Serialize, serde::Deserialize)]
        #input

        impl rpc_rc::Event for #name {
            fn name() -> &'static str {
                #name_str
            }
        }
    }
    .to_token_stream()
    .into()
}
