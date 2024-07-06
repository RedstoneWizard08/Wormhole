use prisma_client_rust_sdk::prisma::prisma_models::{
    walkers::{ModelWalker, RefinedFieldWalker},
    FieldArity,
};

use crate::prelude::*;

pub fn r#struct(model: ModelWalker) -> TokenStream {
    let struct_fields = model
        .fields()
        .filter(|f| f.ast_field().field_type.as_unsupported().is_none())
        .filter_map(|field| match field.refine() {
            RefinedFieldWalker::Scalar(field) => {
                let pk = field.is_autoincrement()
                    || field.is_single_pk()
                    || field.is_part_of_a_compound_pk();

                let field_name_str = field.name();
                let field_name_snake = snake_ident(field_name_str);
                let optional = field.ast_field().arity == FieldArity::Optional;

                Some(if optional {
                    quote! {
                        #[serde(
                            rename = #field_name_str,
                            skip_serializing_if = "Option::is_none"
                        )]
                        pub #field_name_snake: #field_name_snake::Type
                    }
                } else if pk {
                    quote! {
                        #[serde(rename = #field_name_str)]
                        pub #field_name_snake: #field_name_snake::Type
                    }
                } else {
                    quote! {
                        #[serde(
                            rename = #field_name_str,
                            skip_serializing_if = "Option::is_none"
                        )]
                        pub #field_name_snake: Option<#field_name_snake::Type>
                    }
                })
            }
            _ => None,
        });

    let struct_set_params = model
        .fields()
        .filter(|f| f.ast_field().field_type.as_unsupported().is_none())
        .filter_map(|field| match field.refine() {
            RefinedFieldWalker::Scalar(field) => {
                let pk = field.is_autoincrement()
                    || field.is_single_pk()
                    || field.is_part_of_a_compound_pk();

                let field_name_str = field.name();
                let field_name_snake = snake_ident(field_name_str);
                let optional = field.ast_field().arity == FieldArity::Optional;

                if pk {
                    return None;
                }

                Some(if optional {
                    quote! {
                        if let Some(field) = self.#field_name_snake {
                            params.push(#field_name_snake::set(Some(field)));
                        }
                    }
                } else {
                    quote! {
                        if let Some(field) = self.#field_name_snake {
                            params.push(#field_name_snake::set(field));
                        }
                    }
                })
            }
            _ => None,
        });

    let specta_derive = cfg!(feature = "specta").then(|| {
        let model_name_pascal_str = format!("{}Update", pascal_ident(model.name()));

        quote! {
            #[derive(::prisma_client_rust::specta::Type)]
            #[specta(rename = #model_name_pascal_str, crate = prisma_client_rust::specta)]
        }
    });

    quote! {
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
        #specta_derive
        pub struct Update {
            #(#struct_fields),*
        }

        impl Update {
            pub fn as_params(self) -> Vec<SetParam> {
                let mut params = Vec::new();

                #(#struct_set_params)*

                params
            }
        }
    }
}
