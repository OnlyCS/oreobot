#![feature(let_chains, proc_macro_span)]

use proc_macro::{self, TokenStream};
use syn::{parse_macro_input, DeriveInput};

#[cfg(any(
    feature = "update-enum",
    feature = "select-menu-options",
    feature = "wire",
    feature = "logger-wire"
))]
mod common;

#[cfg(feature = "update-enum")]
mod update_enum;

#[cfg(feature = "select-menu-options")]
mod select_menu_options;

#[cfg(feature = "wire")]
mod wire;

#[cfg(feature = "logger-wire")]
mod logger_wire;

#[cfg(feature = "update-enum")]
#[proc_macro_attribute]
pub fn update_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    update_enum::proc(input, proc_macro2::TokenStream::from(args)).into()
}

#[cfg(feature = "select-menu-options")]
#[proc_macro_derive(SelectMenuOptions, attributes(label, ty))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { data, ident, .. } = parse_macro_input!(input as DeriveInput);
    select_menu_options::proc(data, ident).into()
}

#[cfg(feature = "wire")]
#[proc_macro]
pub fn wire(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as proc_macro2::TokenStream);

    wire::proc(input).into()
}

#[cfg(feature = "logger-wire")]
#[proc_macro]
pub fn logger_wire(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as proc_macro2::TokenStream);

    logger_wire::proc(input).into()
}

#[cfg(feature = "autocommand")]
#[proc_macro]
pub fn autocommand(_: TokenStream) -> TokenStream {
    let file = proc_macro::Span::call_site().source_file();
    let path = file.path();

    // get filenames of every file inside dir
    let files = std::fs::read_dir(path.parent().unwrap())
        .unwrap()
        .map(|res| {
            res.map(|e| {
                e.path()
                    .to_str()
                    .unwrap()
                    .split("/")
                    .last()
                    .unwrap()
                    .trim()
                    .trim_end_matches(".rs")
                    .to_owned()
            })
        })
        .filter_map(|res| res.ok())
        .filter(|file| file != "mod")
        .map(|n| proc_macro2::Ident::new(&n, proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();

    quote::quote! {
        #(mod #files;)*

        pub fn all() -> Vec<poise::Command<crate::prelude::Data, crate::prelude::CommandError>> {
            vec![
                #(#files::#files()),*
            ]
        }
    }
    .into()
}

#[cfg(feature = "from-prisma-error")]
#[proc_macro_derive(FromPrismaError)]
pub fn from_prisma_error(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let err = input.ident;

    quote::quote! {
        impl From<prisma_client_rust::NewClientError> for #err {
            fn from(value: prisma_client_rust::NewClientError) -> Self {
                Self::from(prisma::Error::from(value))
            }
        }

        impl From<prisma_client_rust::QueryError> for #err {
            fn from(value: prisma_client_rust::QueryError) -> Self {
                Self::from(prisma::Error::from(value))
            }
        }

        impl From<prisma_client_rust::RelationNotFetchedError> for #err {
            fn from(value: prisma_client_rust::RelationNotFetchedError) -> Self {
                Self::from(prisma::Error::from(value))
            }
        }
    }
    .into()
}
