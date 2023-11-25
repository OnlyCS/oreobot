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
