use itertools::Itertools;
use prisma_client_rust_sdk::prisma::{
    prisma_models::walkers::ModelWalker, psl::parser_database::ScalarFieldType,
};

use super::required_fields;
use crate::prelude::*;

fn create_unchecked(model: ModelWalker) -> Option<TokenStream> {
    required_fields(model)?;

    let model_name_snake = snake_ident(model.name());

    let (names, serdes, types): (Vec<_>, Vec<_>, Vec<_>) = model
        .scalar_fields()
        .filter_map(|field| {
            let field_name_str = field.name();
            let name_snake = snake_ident(field.name());
            let serde_str = quote!(#[serde(rename = #field_name_str)]);

            if !field.required_on_create() {
                return None;
            }

            Some((
                name_snake,
                serde_str,
                match field.scalar_field_type() {
                    ScalarFieldType::CompositeType(id) => {
                        let comp_type = model.db.walk(id);

                        let comp_type_snake = snake_ident(comp_type.name());

                        quote!(super::#comp_type_snake::Create)
                    }
                    _ => field.type_tokens(&quote!(super::))?,
                },
            ))
        })
        .multiunzip();

    let specta_derive = cfg!(feature = "specta").then(|| {
        let model_name_pascal_str = format!("{}Creation", pascal_ident(model.name()));

        quote! {
            #[derive(::prisma_client_rust::specta::Type)]
            #[specta(rename = #model_name_pascal_str, crate = prisma_client_rust::specta)]
        }
    });

    Some(quote! {
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        #specta_derive
        pub struct CreateUnchecked {
            #(
                #serdes
                pub #names: #types,
            )*

            #[serde(skip)]
            pub _params: Vec<UncheckedSetParam>
        }

        impl CreateUnchecked {
             pub fn to_query<'a>(self, client: &'a PrismaClient) -> CreateUncheckedQuery<'a> {
                client.#model_name_snake()
                    .create_unchecked(
                        #(self.#names,)*
                        self._params
                    )
            }

            pub fn to_params(mut self) -> Vec<UncheckedSetParam> {
                self._params.extend([
                    #(#names::set(self.#names)),*
                ]);

                self._params
            }
        }

        pub fn create_unchecked(#(#names: #types,)* _params: Vec<UncheckedSetParam>)
            -> CreateUnchecked {
            CreateUnchecked {
                #(#names,)*
                _params
            }
        }
    })
}

fn create(model: ModelWalker) -> Option<TokenStream> {
    let model_name_snake = snake_ident(model.name());

    let (names, (types, push_wrappers)): (Vec<_>, (Vec<_>, Vec<_>)) = required_fields(model)?
        .into_iter()
        .map(|field| {
            (
                snake_ident(field.inner.name()),
                (field.typ, field.push_wrapper),
            )
        })
        .unzip();

    Some(quote! {
       #[derive(Debug, Clone)]
        pub struct Create {
            #(pub #names: #types,)*
            pub _params: Vec<SetParam>
        }

        impl Create {
            pub fn to_query<'a>(self, client: &'a PrismaClient) -> CreateQuery<'a> {
                client.#model_name_snake()
                    .create(
                        #(self.#names,)*
                        self._params
                    )
            }

            pub fn to_params(mut self) -> Vec<SetParam> {
                self._params.extend([
                    #(#names::#push_wrappers(self.#names)),*
                ]);

                self._params
            }
        }

        pub fn create(#(#names: #types,)* _params: Vec<SetParam>)
            -> Create {
            Create {
                #(#names,)*
                _params
            }
        }
    })
}

pub fn types(model: ModelWalker) -> TokenStream {
    let create_unchecked = create_unchecked(model);
    let create = create(model);

    quote! {
        #create

        #create_unchecked
    }
}
