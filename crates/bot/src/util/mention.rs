pub enum MentionType {
    User,
    Role,
    Channel,
}

pub fn create(id: impl std::fmt::Display, mention_type: MentionType) -> String {
    match mention_type {
        MentionType::User => format!("<@{}>", id),
        MentionType::Role => format!("<@&{}>", id),
        MentionType::Channel => format!("<#{}>", id),
    }
}
