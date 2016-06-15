use rss;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TextInput {
    pub title: String,
    pub description: String,
    pub name: String,
    pub link: String,
}

impl From<rss::TextInput> for TextInput {
    fn from(input: rss::TextInput) -> TextInput {
        TextInput {
            title: input.title,
            description: input.description,
            name: input.name,
            link: input.link,
        }
    }
}

impl From<TextInput> for rss::TextInput {
    fn from(input: TextInput) -> rss::TextInput {
        rss::TextInput {
            title: input.title,
            description: input.description,
            name: input.name,
            link: input.link,
        }
    }
}
