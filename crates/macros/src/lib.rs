#[macro_use]
extern crate quote;

mod handler;

use handler::Handler;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{ext::IdentExt, parse_macro_input, FnArg, ImplItemFn, ItemFn, ItemImpl, Pat, Path};

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

    (quote! {
        #func
        #obj
        #wrapper
    }).into()
}

#[proc_macro]
pub fn serde_funcs(item: TokenStream) -> TokenStream {
    parse_macro_input!(item as Handler).into()
}

#[proc_macro_attribute]
pub fn serde_impl(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let it = parse_macro_input!(item as ItemImpl);
    let mut out: Vec<proc_macro2::TokenStream> = Vec::new();
    let stream: TokenStream = it.self_ty.to_token_stream().into();
    let holder = parse_macro_input!(stream as Path);
    let holder_ident = holder.segments.last().unwrap().clone().ident;
    let instance = format_ident!("__instance_{}", holder_ident);

    let lazy = quote! {
        lazy_static::lazy_static! {
            static ref #instance: std::sync::Arc<std::sync::Mutex<#holder>> = std::sync::Arc::new(std::sync::Mutex::new(#holder::new()));
        }
    };

    for item in &it.items {
        let s: TokenStream = item.to_token_stream().into();
        let func = parse_macro_input!(s as ImplItemFn);

        if func.sig.ident.to_string() == "new" {
            continue;
        }

        out.push(serde_struct_call_inner(&holder, item.to_token_stream().into()).into());
    }

    quote! {
        #it
        #lazy
        #(#out)*
    }.into()
}

fn serde_struct_call_inner(struct_holder: &Path, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ImplItemFn);
    let is_async = func.sig.asyncness.is_some();
    let name = func.sig.ident.clone();
    let holder = format_ident!("__serde_args_{}", name.clone());
    let invoke = format_ident!("__serde_invoke_{}", name.clone());
    let ret = func.sig.output.clone();
    let vis = func.vis.clone();
    let async_ = if is_async { quote!(async) } else { quote!() };
    let await_ = if is_async { quote!(.await) } else { quote!() };
    let holder_ident = struct_holder.segments.last().unwrap().clone().ident;
    let instance = format_ident!("__instance_{}", holder_ident);

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
        #vis #async_ fn #invoke(data: impl AsRef<str>) #ret {
            let holder = serde_json::from_str::<#holder>(data.as_ref()).unwrap();

            #struct_holder::#name(&#instance.lock().unwrap(), #(#accessors,)*)#await_
        }
    };

    (quote! {
        #func
        #obj
        #wrapper
    }).into()
}
