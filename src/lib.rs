use std::str::FromStr;

#[derive(Clone)]
pub enum Feed {
    Atom(atom_syndication::Feed),
    RSS(rss::Channel),
}

impl FromStr for Feed {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match atom_syndication::Feed::from_str(s) {
            Ok(feed) => Ok(Feed::Atom(feed)),
            _ => match rss::Channel::from_str(s) {
                Ok(feed) => Ok(Feed::RSS(feed)),
                _ => Err("Could not parse XML as Atom or RSS from input"),
            },
        }
    }
}

impl ToString for Feed {
    fn to_string(&self) -> String {
        match self {
            Feed::Atom(atom_feed) => atom_feed.to_string(),
            Feed::RSS(rss_channel) => rss_channel.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;

    use super::Feed;

    // Source: https://github.com/vtduncan/rust-atom/blob/master/src/lib.rs
    #[test]
    fn test_from_atom_file() {
        let mut file = File::open("test-data/atom.xml").unwrap();
        let mut atom_string = String::new();
        file.read_to_string(&mut atom_string).unwrap();
        let feed = Feed::from_str(&atom_string).unwrap();
        assert!(!feed.to_string().is_empty());
    }

    // Source: https://github.com/frewsxcv/rust-rss/blob/master/src/lib.rs
    #[test]
    fn test_from_rss_file() {
        let mut file = File::open("test-data/rss.xml").unwrap();
        let mut rss_string = String::new();
        file.read_to_string(&mut rss_string).unwrap();
        let rss = Feed::from_str(&rss_string).unwrap();
        assert!(!rss.to_string().is_empty());
    }

    // Source: https://github.com/vtduncan/rust-atom/blob/master/src/lib.rs
    #[test]
    fn test_atom_to_string() {
        let author = atom_syndication::PersonBuilder::default()
            .name("N. Blogger")
            .build();

        let entry = atom_syndication::EntryBuilder::default()
            .title("My first post!")
            .content(Some(
                atom_syndication::ContentBuilder::default()
                    .value(Some("This is my first post".to_string()))
                    .build(),
            ))
            .build();

        let feed = atom_syndication::FeedBuilder::default()
            .title("My Blog")
            .authors(vec![author])
            .entries(vec![entry])
            .build();

        assert_eq!(feed.to_string(), "<?xml version=\"1.0\"?>\n<feed xmlns=\"http://www.w3.org/2005/Atom\"><title>My Blog</title><id></id><updated>1970-01-01T00:00:00+00:00</updated><author><name>N. Blogger</name></author><entry><title>My first post!</title><id></id><updated>1970-01-01T00:00:00+00:00</updated><content>This is my first post</content></entry></feed>");
    }

    // Source: https://github.com/frewsxcv/rust-rss/blob/master/src/lib.rs
    #[test]
    fn test_rss_to_string() {
        let item = rss::ItemBuilder::default()
            .title(Some("My first post!".to_string()))
            .link(Some("http://myblog.com/post1".to_string()))
            .description(Some("This is my first post".to_string()))
            .build();

        let channel = rss::ChannelBuilder::default()
            .title("My Blog")
            .link("http://myblog.com")
            .description("Where I write stuff")
            .items(vec![item])
            .build();

        let rss = Feed::RSS(channel);
        assert_eq!(rss.to_string(), "<?xml version=\"1.0\" encoding=\"utf-8\"?><rss version=\"2.0\"><channel><title>My Blog</title><link>http://myblog.com</link><description>Where I write stuff</description><item><title>My first post!</title><link>http://myblog.com/post1</link><description><![CDATA[This is my first post]]></description></item></channel></rss>");
    }
}
