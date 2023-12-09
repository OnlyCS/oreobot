use proc_macro2::TokenStream;

use crate::common::*;

pub(crate) fn proc(ts: TokenStream) -> TokenStream {
    let mut trees = ts.into_iter();

    let ident_match = expect_ret!(trees, TokenTree::Ident(i), { i });
    expect!(trees, TokenTree::Punct(comma), { comma.as_char() == ',' });

    let ident_channel = expect_ret!(trees, TokenTree::Ident(i), { i });

    let mut arms = vec![];

    while let Some(tree) = trees.next() {
        expect_for!(tree, TokenTree::Punct(p), { p.as_char() == ':' });

        let group = expect! {
            trees,
            TokenTree::Group(braced),
            {braced.delimiter() == Delimiter::Brace},
            {braced}
        };

        let mut group_trees = group.stream().into_iter();

        while let Some(tree) = group_trees.next() {
            let ident = expect_for! {
                tree,
                TokenTree::Ident(i),
                {i}
            };

            let mut arm = quote!();
            let mut body = quote!();
            let mut and_then = quote!();

            match ident.to_string().as_str() {
                "data" => {
                    // data: Something::Else { data, .. },

                    let _arm = until(&mut group_trees, |tree| match tree {
                        TokenTree::Punct(p) => p.as_char() == ',',
                        _ => false,
                    });

                    arm = quote! {
                        serenity::FullEvent::#_arm
                    }
                }
                "request" => {
                    // request: MessageCreate(data),
                    let _request = until(&mut group_trees, |tree| match tree {
                        TokenTree::Punct(p) => p.as_char() == ',',
                        _ => false,
                    });

                    let request = quote! {
                        LoggingRequest::#_request
                    };

                    body = quote! {
                        let _chk = #ident_channel.send(#request).await?;
                    };
                }
                "and_then" => {
                    // should be a closure
                    let closure = until(&mut group_trees, |tree| match tree {
                        TokenTree::Punct(p) => p.as_char() == ',',
                        _ => false,
                    });

                    and_then = quote! {
                        (#closure)(_chk);
                    }
                }
                _ => panic!("Unknown field: {}", ident.to_string()),
            }

            body = quote! {
                #body
                #and_then
            };

            arms.push(quote! {
                #arm => {
                    #body
                }
            });
        }
    }

    quote! {
        match #ident_match {
            #(#arms)*
            _ => {}
        }
    }
}
