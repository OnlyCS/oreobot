use crate::prelude::*;

pub trait StringUtil {
    fn capitalize_first_letter(&self) -> String;
    fn display_case(&self) -> String;
}

impl<T> StringUtil for T
where
    T: ToString,
{
    fn capitalize_first_letter(&self) -> String {
        let string = self.to_string();
        let mut chars = string.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    fn display_case(&self) -> String {
        self.to_string()
            .split("_")
            .into_iter()
            .map(|n| n.capitalize_first_letter())
            .join(" ")
    }
}
