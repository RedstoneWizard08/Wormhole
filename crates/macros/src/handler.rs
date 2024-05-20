// MIT License
//
// Copyright (c) 2024 Jacob Sapoznikow <jacob1coder@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. 

use std::collections::HashMap;

use syn::{
    parse::{Parse, ParseBuffer},
    Ident, Path, PathSegment, Token,
};

pub fn path_to_command(path: &mut Path) -> &mut PathSegment {
    path.segments
        .last_mut()
        .expect("parsed syn::Path has no segment")
}

pub fn format_command_wrapper(function: &Ident) -> Ident {
    format_ident!("__serde_invoke_{}", function)
}

pub struct Handler {
    pub items: HashMap<Ident, Path>,
}

impl Parse for Handler {
    fn parse(input: &ParseBuffer<'_>) -> syn::Result<Self> {
        let paths = input.parse_terminated(Path::parse, Token![,])?;

        // parse the command names and wrappers from the passed paths
        let items = paths
            .iter()
            .map(|path| {
                let mut wrapper = path.clone();
                let last = path_to_command(&mut wrapper);

                // the name of the actual command function
                let cmd = last.ident.clone();

                // set the path to the command function wrapper
                last.ident = format_command_wrapper(&cmd);

                (cmd, wrapper)
            })
            .collect::<HashMap<Ident, Path>>();

        Ok(Self { items })
    }
}

impl From<Handler> for proc_macro::TokenStream {
    fn from(Handler { items }: Handler) -> Self {
        let mut out = Vec::new();

        for (cmd, path) in items {
            let cmd = cmd.to_string();

            out.push(quote!(#cmd => serde_json::to_string(&#path(data, state).await).unwrap()));
        }

        quote! {
            pub async fn handle_serde_call(cmd: impl AsRef<str>, data: impl AsRef<str>, state: AppState<'_>) -> String {
                let cmd = cmd.as_ref();

                match cmd {
                    #(#out,)*

                    _ => serde_json::to_string(&serde_json::json!({
                        "error": "UNKNOWN_COMMAND",
                        "status": 500,
                    })).unwrap(),
                }
            }
        }.into()
    }
}
