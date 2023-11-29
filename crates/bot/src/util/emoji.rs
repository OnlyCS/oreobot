pub const CURVED: &str = "<:1178725162464596038:curved>";
pub const STRAIGHT: &str = "<:1178725321122512926:straight>";

pub fn create(id: impl Into<String>, name: impl Into<String>) -> String {
    format!("<:{}:{}>", id.into(), name.into())
}
