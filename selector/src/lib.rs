use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Data, DataStruct};

#[proc_macro_derive(Selector)]
pub fn selector_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("parsing failed");
    impl_selector_derive(&ast)
}

fn impl_selector_derive(ast: &syn::DeriveInput) -> TokenStream {
    let struct_identifier = &ast.ident;
    match &ast.data {
        Data::Struct(DataStruct { fields, .. }) => {
            let mut implementation = quote! {
                let mut hash_map = HashMap::<String, String>::new();
            };

            for field in fields {
                let identifier = field.ident.as_ref().unwrap();
                implementation.extend(quote! {
                    hash_map.insert(stringify!(#identifier).to_string(), String::from(value.#identifier));
                });
            }

            quote! {
                #[automatically_derived]
                impl From<#struct_identifier> for HashMap<String, String> {
                    fn from(value: #struct_identifier) -> Self {
                        #implementation

                        hash_map
                    }
                }
            }
        }
        _ => unimplemented!(),
    }
    .into()
}
