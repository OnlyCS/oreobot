use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn proc(input: DeriveInput, args_ts: TokenStream) -> TokenStream {
    let data = &input.data;
    let ident = &input.ident;
    let mut rewritten_idents = vec![];
    let mut rewritten_types = vec![];
    let mut original_idents = vec![];

    let syn::Data::Struct(s) = data else {
        panic!("update_enum can only be derived for structs");
    };

    let fields = &s.fields;

    for field in fields {
        let ty = &field.ty;
        original_idents.push(field.ident.as_ref().unwrap().clone());
        let ident = field.ident.as_ref().unwrap().to_string();
        let ident_camel = ident
            .split('_')
            .map(|spl| {
                spl.chars()
                    .enumerate()
                    .map(|(idx, c)| if idx == 0 { c.to_ascii_uppercase() } else { c })
                    .collect::<String>()
            })
            .collect::<String>();

        let new_ident = syn::Ident::new(&ident_camel, field.ident.as_ref().unwrap().span());

        rewritten_idents.push(new_ident.clone());
        rewritten_types.push(ty.clone());
    }

    let new_ident = syn::Ident::new(&format!("Update{}", ident.to_string()), ident.span());

    quote! {
        #args_ts
        #input

        pub enum #new_ident {
            #(#rewritten_idents (#rewritten_types)),*
        }

        impl #new_ident {
            pub fn update_with(self, to_update: &mut #ident) {
                match self {
                    #(#new_ident::#rewritten_idents(n) => to_update.#original_idents = n),*
                }
            }
        }

        impl #ident {
            pub fn update_from(&mut self, update: #new_ident) {
                update.update_with(self);
            }
        }
    }
}
