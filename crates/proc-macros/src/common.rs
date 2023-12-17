pub use itertools::Itertools;
pub use proc_macro2::{TokenStream, TokenTree};
pub use quote::quote;

pub(crate) fn until<T: Iterator<Item = TokenTree> + Clone>(
    stream: &mut T,
    stop_when: impl Fn(&TokenTree) -> bool,
) -> TokenStream {
    let tree = stream.take_while_ref(|tree| !stop_when(tree)).collect_vec();
    TokenStream::from_iter(tree)
}

pub(crate) fn until_multi_punct(
    stream: &mut (impl Iterator<Item = TokenTree> + Clone),
    punct_a: char,
    punct_b: char,
) -> TokenStream {
    let data = until(stream, |tree| match tree {
        TokenTree::Punct(p) => p.as_char() == punct_a,
        _ => false,
    });

    let punct_a_tree = stream.next().unwrap();

    if let Some(next) = stream.next() {
        if let TokenTree::Punct(p) = &next
            && p.as_char() == punct_b
        {
            data
        } else {
            let continuation = until_multi_punct(stream, punct_a, punct_b);

            data.into_iter()
                .chain(std::iter::once(punct_a_tree))
                .chain(std::iter::once(next))
                .chain(continuation)
                .collect()
        }
    } else {
        panic!("Expected `{}` at ({}, {})", punct_b, line!(), column!());
    }
}

pub(crate) fn snake_to_pascal(s: impl Into<String>) -> String {
    let s = s.into();
    s.split('_')
        .map(|spl| {
            spl.chars()
                .enumerate()
                .map(|(idx, c)| if idx == 0 { c.to_ascii_uppercase() } else { c })
                .collect::<String>()
        })
        .collect::<String>()
}

macro_rules! expect_mac {
    ($tree:expr, $expected:pat, $check:block) => {
        expect!($tree, $expected, $check, {})
    };
    ($tree:expr, comma) => {
        expect!($tree, TokenTree::Punct(p), { p.as_char() == ',' })
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

macro_rules! expect_on {
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
    ($tree:expr, comma) => {
        expect_on!($tree, TokenTree::Punct(p), { p.as_char() == ',' })
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

pub(crate) use expect_mac as expect;
pub(crate) use expect_on;
pub(crate) use expect_ret;
