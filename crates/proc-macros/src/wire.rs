use crate::common::*;

pub(crate) fn proc(ts: TokenStream) -> TokenStream {
    let mut trees = ts.into_iter();

    let match_item = expect_ret!(trees, TokenTree::Ident(i), { i });

    let mut arms = vec![];

    while let Some(tree) = trees.next() {
        expect_on!(tree, comma);

        let data = until_multi_punct(&mut trees, '=', '>');
        let handler = until_multi_punct(&mut trees, '=', '>');
        let response = until(&mut trees, |tree| match tree {
            TokenTree::Punct(p) if p.as_char() == ',' => true,
            _ => false,
        });

        if handler.to_string() == "_" {
            arms.push(quote! {
                LoggingRequest::#data => {
                    Ok(#response)
                }
            });

            continue;
        }

        arms.push(quote! {
            LoggingRequest::#data => {
                let out = #handler.await?;
                Ok(#response)
            }
        })
    }

    quote! {
        match #match_item {
            #(#arms),*,
            _ => panic!("No match"),
        }
    }
}
