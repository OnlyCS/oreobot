use proc_macro::{self, TokenStream};
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[cfg(feature = "update-enum")]
mod update_enum;

#[cfg(feature = "select-menu-options")]
mod select_menu_options;

#[cfg(feature = "wire")]
mod wire;

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

#[proc_macro]
pub fn wire(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as proc_macro2::TokenStream);

    wire::proc(input).into()
}
