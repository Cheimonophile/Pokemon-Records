extern crate proc_macro;

use core::panic;

use proc_macro::*;

use quote::quote;

#[proc_macro_derive(FromFlatSqlx)]
pub fn derive_from_flat_sqlx(_input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, .. } = syn::parse_macro_input!(_input);

    let x: TokenStream = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(fields) => {
                for field in fields.named {
                    match field.ident {
                        None => panic!("Only named fields are supported"),
                        Some(ident) => {}
                    }
                }
                quote! {
                  struct #ident {

                  }
                }
                .into()
            }
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // print the macro
    let trait_name_str = format!("__TraitFromFlatSqlx{}", ident);
    let trait_name = syn::Ident::new(&trait_name_str, Span::call_site().into());
    let struct_name_str = format!("__StructFromFlatSqlx{}", ident);
    let struct_name = syn::Ident::new(&struct_name_str, Span::call_site().into());
    let out = quote! {
        impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for #ident {
            fn from_row(row: &sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
                Ok(#ident {
                    // id: row.try_get("id")?,
                    // name: row.try_get("name")?,
                })
            }
        }
    };

    out.into()
}
