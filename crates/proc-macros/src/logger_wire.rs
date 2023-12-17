use crate::common::*;

pub(crate) fn proc(ts: TokenStream) -> TokenStream {
    let mut trees = ts.into_iter();

    let ident_match = expect_ret!(trees, TokenTree::Ident(i), { i });
    expect!(trees, comma);

    let ident_channel = expect_ret!(trees, TokenTree::Ident(i), { i });

    let mut arms = vec![];

    while let Some(tree) = trees.next() {
        expect_on!(tree, comma);

        let data = until_multi_punct(&mut trees, '=', '>');
        let request = until(&mut trees, |tree| match tree {
            TokenTree::Punct(p) => p.as_char() == ',',
            _ => false,
        });

        if data.to_string() == "_" {
            arms.push(quote! {
                _ => #request,
            });

            continue;
        }

        arms.push(quote! {
            serenity::FullEvent::#data => {
                let response = #ident_channel.send(LoggingRequest::#request).await?;
            }
        });
    }

    quote! {
        match #ident_match {
            #(#arms)*
            _ => {}
        }
    }
}
