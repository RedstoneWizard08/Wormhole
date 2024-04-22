#[macro_use]
extern crate quote;

mod handler;

use handler::Handler;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{ext::IdentExt, parse_macro_input, FnArg, ItemFn, ItemImpl, Pat};

#[proc_macro_attribute]
pub fn serde_call(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let is_async = func.sig.asyncness.is_some();
    let name = func.sig.ident.clone();
    let holder = format_ident!("__serde_args_{}", name.clone());
    let invoke = format_ident!("__serde_invoke_{}", name.clone());
    let ret = func.sig.output.clone();
    let vis = func.vis.clone();
    let async_ = if is_async { quote!(async) } else { quote!() };
    let await_ = if is_async { quote!(.await) } else { quote!() };

    let mut fields = Vec::new();
    let mut accessors = Vec::new();

    for arg in func.sig.inputs.clone() {
        if let FnArg::Typed(arg) = arg {
            let stream = arg.to_token_stream();

            let arg_name = match arg.pat.as_ref().clone() {
                Pat::Ident(id) => id.ident.unraw(),
                _ => panic!("Needs an ident!"),
            };

            fields.push(stream);
            accessors.push(quote!(holder.#arg_name));
        }
    }

    fields.pop();
    accessors.pop();

    let obj = quote! {
        #[allow(non_camel_case_types)]
        #[derive(serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct #holder {
            #(pub #fields,)*
        }
    };

    let wrapper = quote! {
        #vis #async_ fn #invoke(data: impl AsRef<str>, state: AppState<'_>) #ret {
            let holder = serde_json::from_str::<#holder>(data.as_ref()).unwrap();

            #name(#(#accessors,)* state)#await_
        }
    };

    quote! {
        #func
        #obj
        #wrapper
    }
    .into()
}

#[proc_macro]
pub fn serde_funcs(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as Handler).into()
}

#[proc_macro_attribute]
pub fn serde_impl(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let _impl_ = parse_macro_input!(item as ItemImpl);

    todo!()
}
