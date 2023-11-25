use crate::common::snake_to_pascal;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn proc(input: DeriveInput, args_ts: TokenStream) -> TokenStream {
    let data = &input.data;
    let ident = &input.ident;
    let mut fields_pascal = vec![];
    let mut field_types = vec![];
    let mut fields_original = vec![];

    let syn::Data::Struct(s) = data else {
        panic!("update_enum can only be derived for structs");
    };

    let fields = &s.fields;

    for field in fields {
        fields_original.push(field.ident.as_ref().unwrap().clone());

        let ty = &field.ty;
        let ident_pascal = syn::Ident::new(
            &snake_to_pascal(&field.ident.as_ref().unwrap().to_string()),
            field.ident.as_ref().unwrap().span(),
        );

        fields_pascal.push(ident_pascal);
        field_types.push(ty);
    }

    let update_ident = syn::Ident::new(&format!("Update{}", ident.to_string()), ident.span());

    quote! {
        #args_ts
        #input

        #[derive(Clone, Debug, Serialize, Deserialize)]
        pub enum #update_ident {
            #(#fields_pascal (#field_types)),*
        }

        impl #update_ident {
            pub fn update_with(self, to_update: &mut #ident) {
                match self {
                    #(#update_ident::#fields_pascal(n) => to_update.#fields_original = n),*
                }
            }
        }

        impl #ident {
            pub fn update_from(&mut self, update: #update_ident) {
                update.update_with(self);
            }
        }
    }
}
