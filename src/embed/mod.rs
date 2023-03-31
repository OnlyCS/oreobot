use crate::prelude::*;

pub trait EmbedAdditions {
    fn make(&mut self, data: Data) -> &mut Self;
    fn make_default(&mut self, data: Data) -> &mut Self;
    fn make_error(&mut self, data: Data) -> &mut Self;
    fn make_success(&mut self, data: Data) -> &mut Self;
    fn make_warning(&mut self, data: Data) -> &mut Self;
    fn data(&mut self, title: &str, description: &str) -> &mut Self;
}

impl EmbedAdditions for CreateEmbed {
    fn make(&mut self, data: Data) -> &mut Self {
        self.timestamp(now())
            .footer(|f| f.text("Smarty").icon_url(data.bot_icon))
    }

    fn make_default(&mut self, data: Data) -> &mut Self {
        self.make(data).color(Color::Primary)
    }

    fn make_error(&mut self, data: Data) -> &mut Self {
        self.make(data).color(Color::Error)
    }

    fn make_success(&mut self, data: Data) -> &mut Self {
        self.make(data).color(Color::Success)
    }

    fn make_warning(&mut self, data: Data) -> &mut Self {
        self.make(data).color(Color::Warning)
    }

    fn data(&mut self, title: &str, description: &str) -> &mut Self {
        self.title(title).description(description)
    }
}
