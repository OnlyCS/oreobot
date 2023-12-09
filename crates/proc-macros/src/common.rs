pub use itertools::Itertools;
pub use proc_macro2::{Delimiter, Ident, Span, TokenStream, TokenTree};
pub use quote::quote;

pub(crate) fn until<T: Iterator<Item = TokenTree> + Clone>(
    stream: &mut T,
    stop_when: impl Fn(&TokenTree) -> bool,
) -> TokenStream {
    let tree = stream.take_while_ref(|tree| !stop_when(tree)).collect_vec();
    TokenStream::from_iter(tree)
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

pub(crate) fn pascal_to_snake(s: impl Into<String>) -> String {
    let s = s.into();
    s.split(|n: char| n.is_ascii_uppercase())
        .map(|spl| spl.to_ascii_lowercase())
        .collect::<Vec<String>>()
        .join("_")
}

macro_rules! expect_mac {
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

pub(crate) use expect_for;
pub(crate) use expect_mac as expect;
pub(crate) use expect_ret;
