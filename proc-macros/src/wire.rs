use crate::common::snake_to_pascal;
use itertools::Itertools;
use proc_macro2::{Delimiter, Ident, Span, TokenStream, TokenTree};
use quote::quote;

macro_rules! expect {
    ($tree:expr, $expected:pat, $check:block) => {
        match $tree.next().unwrap() {
            $expected => {
                if !$check {
                    panic!(
                        "Expected {} such that {}, at ({}, {})",
                        stringify!($expected),
                        stringify!($check),
                        line!(),
                        column!()
                    );
                }
            }
            _ => panic!(
                "Expected {}, at ({}, {})",
                stringify!($expected),
                line!(),
                column!()
            ),
        }
    };
    ($tree:expr, $expected:pat, $check:block, $ret:block) => {
        match $tree.next().unwrap() {
            $expected => {
                if !$check {
                    panic!(
                        "Expected {} such that {}, at ({}, {})",
                        stringify!($expected),
                        stringify!($check),
                        line!(),
                        column!()
                    );
                } else {
                    $ret
                }
            }
            _ => panic!(
                "Expected {}, at ({}, {})",
                stringify!($expected),
                line!(),
                column!()
            ),
        }
    };
}

macro_rules! expect_for {
    ($tree:expr, $expected:pat, $ret:block) => {
        match $tree {
            $expected => $ret,
            _ => panic!(
                "Expected {}, at ({}, {})",
                stringify!($expected),
                line!(),
                column!()
            ),
        }
    };
}

macro_rules! expect_ret {
    ($tree:expr, $expected:pat, $ret:block) => {
        match $tree.next().unwrap() {
            $expected => $ret,
            _ => panic!(
                "Expected {}, at ({}, {})",
                stringify!($expected),
                line!(),
                column!()
            ),
        }
    };
}

fn until<T: Iterator<Item = TokenTree> + Clone>(
    stream: &mut T,
    stop_when: impl Fn(&TokenTree) -> bool,
) -> TokenStream {
    let tree = stream.take_while_ref(|tree| !stop_when(tree)).collect_vec();
    TokenStream::from_iter(tree)
}

fn parse_crud(
    trees: &mut impl Iterator<Item = TokenTree>,
    request_enum: &Ident,
) -> Vec<TokenStream> {
    expect!(trees, TokenTree::Punct(p), { p.as_char() == ':' });

    let group = expect! {
        trees,
        TokenTree::Group(braced),
        { braced.delimiter() == Delimiter::Brace },
        { braced }
    };

    let mut group_trees = group.stream().into_iter();

    let mut create = None;
    let mut read = None;
    let mut update = None;
    let mut delete = None;
    let mut all = None;
    let mut ident_lcase = None;
    let mut response = None;
    let mut function_prefix = None;
    let mut request_prefix = None;
    let mut pat_delete = None;
    let mut read_response = None;
    let mut read_response_all = None;

    while let Some(tree) = group_trees.next() {
        let ident = expect_for! {
            tree,
            TokenTree::Ident(i),
            { i }
        };

        match ident.to_string().as_str() {
            "item" => {
                expect!(group_trees, TokenTree::Punct(p), { p.as_char() == ':' });

                let ident = expect_ret! {
                    group_trees,
                    TokenTree::Ident(i),
                    { i }
                };

                let ident_pascal = snake_to_pascal(ident.to_string());

                create = Some(format!("{}Create", ident_pascal));
                read = Some(format!("{}Read", ident_pascal));
                update = Some(format!("{}Update", ident_pascal));
                delete = Some(format!("{}Delete", ident_pascal));
                all = Some(format!("{}ReadAll", ident_pascal));
                ident_lcase = Some(ident.to_string());
            }
            "response" => {
                expect!(group_trees, TokenTree::Punct(p), { p.as_char() == ':' });

                let resp_tt = until(&mut group_trees, |tree| match tree {
                    TokenTree::Punct(p) if p.as_char() == ',' => true,
                    _ => false,
                });

                response = Some(resp_tt);
            }
            "request_prefix" => {
                expect!(group_trees, TokenTree::Punct(p), { p.as_char() == ':' });

                let prefix = until(&mut group_trees, |tree| match tree {
                    TokenTree::Punct(p) if p.as_char() == ',' => true,
                    _ => false,
                });

                request_prefix = Some(prefix.to_string());
            }
            "function_prefix" => {
                expect!(group_trees, TokenTree::Punct(p), { p.as_char() == ':' });

                let prefix = until(&mut group_trees, |tree| match tree {
                    TokenTree::Punct(p) if p.as_char() == ',' => true,
                    _ => false,
                });

                function_prefix = Some(prefix.to_string());
            }
            "pat_delete" => {
                expect!(group_trees, TokenTree::Punct(p), { p.as_char() == ':' });

                let pat = until(&mut group_trees, |tree| match tree {
                    TokenTree::Punct(p) if p.as_char() == ',' => true,
                    _ => false,
                });

                pat_delete = Some(pat);
            }
            "read_response" => {
                expect!(group_trees, TokenTree::Punct(p), { p.as_char() == ':' });

                let resp_tt = until(&mut group_trees, |tree| match tree {
                    TokenTree::Punct(p) if p.as_char() == ',' => true,
                    _ => false,
                });

                read_response = Some(resp_tt);
            }
            "read_all_response" => {
                expect!(group_trees, TokenTree::Punct(p), { p.as_char() == ':' });

                let resp_tt = until(&mut group_trees, |tree| match tree {
                    TokenTree::Punct(p) if p.as_char() == ',' => true,
                    _ => false,
                });

                read_response_all = Some(resp_tt);
            }
            _ => panic!("Expected one of: item, response, request_prefix, function_prefix, read_all_response"),
        }

        expect!(group_trees, TokenTree::Punct(p), { p.as_char() == ',' });
    }

    let _delete = delete.clone();
    let req = |item: Option<String>| {
        let variant = Ident::new(
            &*if request_prefix.is_some() {
                format!(
                    "{}{}",
                    request_prefix.as_ref().unwrap(),
                    item.as_ref().unwrap()
                )
            } else {
                item.as_ref().unwrap().clone()
            },
            Span::call_site(),
        );

        if pat_delete.is_some() && item == _delete {
            quote! {
                #request_enum::#variant #pat_delete
            }
        } else if variant.to_string().ends_with("ReadAll") {
            quote! {
                #request_enum::#variant
            }
        } else {
            quote! {
                #request_enum::#variant(item)
            }
        }
    };

    let function = |item: &'static str| {
        let item_ident = Ident::new(item, Span::call_site());

        let function = if function_prefix.is_some() {
            let function_prefix_ident =
                Ident::new(function_prefix.as_ref().unwrap(), Span::call_site());
            let ident_lcase_ident = Ident::new(ident_lcase.as_ref().unwrap(), Span::call_site());

            quote!(#function_prefix_ident::#ident_lcase_ident::#item_ident)
        } else {
            let ident_lcase_ident = Ident::new(ident_lcase.as_ref().unwrap(), Span::call_site());

            quote!(#ident_lcase_ident::#item_ident)
        };

        if item == "all" {
            quote! {
                #function()
            }
        } else {
            quote! {
                #function(item)
            }
        }
    };

    let arm = |req: TokenStream, func: TokenStream, err: String| {
        quote! {
            #req => {
                match #func.await {
                    Ok(()) => #response,
                    Err(error) => {
                        let error_string = format!("{}: {}", #err, error);
                        error!("{}", error_string);
                        LoggingResponse::Err(error_string)
                    },
                }
            }
        }
    };

    let read_response = read_response.unwrap();

    let req_create = req(create);
    let req_read = req(read);
    let req_update = req(update);
    let req_delete = req(delete);
    let req_all = req(all);

    let function_create = function("create");
    let function_read = function("read");
    let function_update = function("update");
    let function_delete = function("delete");
    let function_all = function("all");

    let err_create = format!("Failed to create {}", ident_lcase.as_ref().unwrap());
    let err_read = format!("Failed to read {}", ident_lcase.as_ref().unwrap());
    let err_update = format!("Failed to update {}", ident_lcase.as_ref().unwrap());
    let err_delete = format!("Failed to delete {}", ident_lcase.as_ref().unwrap());
    let err_all = format!("Failed to read every {}", ident_lcase.as_ref().unwrap());

    let arm_create = arm(req_create, function_create, err_create);
    let arm_update = arm(req_update, function_update, err_update);
    let arm_delete = arm(req_delete, function_delete, err_delete);

    let arm_read = quote! {
        #req_read => {
            match #function_read.await {
                Ok(data) => #read_response,
                Err(error) => {
                    let error_string = format!("{}: {}", #err_read, error);
                    error!("{}", error_string);
                    LoggingResponse::Err(error_string)
                },
            }
        }
    };

    let arm_read_all = if let Some(read_response_all) = read_response_all {
        Some(quote! {
            #req_all => {
                match #function_all.await {
                    Ok(data) => #read_response_all,
                    Err(error) => {
                        let error_string = format!("{}: {}", #err_all, error);
                        error!("{}", error_string);
                        LoggingResponse::Err(error_string)
                    },
                }
            }
        })
    } else {
        None
    };

    if let Some(arm_read_all) = arm_read_all {
        vec![arm_create, arm_read, arm_update, arm_delete, arm_read_all]
    } else {
        vec![arm_create, arm_read, arm_update, arm_delete]
    }
}

fn parse_cr(trees: &mut impl Iterator<Item = TokenTree>, request_enum: &Ident) -> Vec<TokenStream> {
    parse_crud(trees, request_enum)[0..2].to_vec()
}

fn parse_custom<T: Iterator<Item = TokenTree> + Clone>(trees: &mut T) -> TokenStream {
    expect!(trees, TokenTree::Punct(p), { p.as_char() == ':' });

    let rule = until(trees, |tree| match tree {
        TokenTree::Punct(p) if p.as_char() == ',' => true,
        _ => false,
    });

    rule
}

pub(crate) fn proc(ts: TokenStream) -> TokenStream {
    let mut token_trees = ts.into_iter();

    let match_item = expect_ret!(token_trees, TokenTree::Ident(i), { i });

    expect!(token_trees, TokenTree::Punct(comma), {
        comma.as_char() == ','
    });

    let request_enum = expect_ret!(token_trees, TokenTree::Ident(i), { i });

    expect!(token_trees, TokenTree::Punct(comma), {
        comma.as_char() == ','
    });

    let mut in_match_ts = vec![];

    while let Some(tree) = token_trees.next() {
        match tree {
            TokenTree::Ident(i) => match i.to_string().as_str() {
                "crud" => in_match_ts.extend(parse_crud(&mut token_trees, &request_enum)),
                "cr" => in_match_ts.extend(parse_cr(&mut token_trees, &request_enum)),
                "custom" => in_match_ts.push(parse_custom(&mut token_trees)),
                _ => panic!("Expected one of: crud, cr, custom"),
            },
            _ => panic!("Expected ident"),
        };

        expect!(token_trees, TokenTree::Punct(comma), {
            comma.as_char() == ','
        });
    }

    quote! {
        match #match_item {
            #(#in_match_ts),*,
            _ => panic!("Expected one of: crud"),
        }
    }
}
