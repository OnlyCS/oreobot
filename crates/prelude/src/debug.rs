const TRUNC_TO: usize = 150;

pub fn debug_truncated(value: impl std::fmt::Debug) -> String {
    let mut value = format!("{:?}", value);

    if value.len() > TRUNC_TO {
        value.truncate(TRUNC_TO);
        value.push_str("...");
    }

    value
}

pub fn string_truncated_dbg(value: impl ToString) -> String {
    let mut value = value.to_string();

    if value.len() > TRUNC_TO {
        value.truncate(TRUNC_TO);
        value.push_str("...");
    }

    value
}
