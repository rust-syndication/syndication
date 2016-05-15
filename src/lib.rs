extern crate atom_syndication;
extern crate rss;
extern crate chrono;

use std::str::FromStr;
use chrono::{DateTime, UTC};

pub struct Entry { }
pub struct Category { }
pub struct Feed {
    // If created from an RSS or Atom feed, this is the original contents
    source_data: Option<FeedData>,
    // `id` in Atom, not present in RSS
    pub id: Option<String>,
    // `title` in both Atom and RSS
    pub title: String,
    // `subtitle` in Atom, and `description` in RSS (required)
    pub description: Option<String>,
    // `updated` in Atom (required), and `pub_date` or `last_build_date` in RSS
    // TODO: Document which RSS field is preferred
    // This field is required in Atom, but optional in RSS
    pub updated: Option<DateTime<UTC>>,
    // `rights` in Atom, and `copyright` in RSS
    pub copyright: Option<String>,
    // `icon` in Atom, and `image` in Atom
    pub icon: Option<String>,
    // `links` in Atom, and `link` in RSS (will produce a Vec of 1 item)
    pub links: Vec<String>,
    // `categories` in both Atom and RSS
    pub categories: Vec<Category>,
    // `authors` in Atom, not present in RSS (RSS will produce an empty Vec)
    pub authors: Vec<atom_syndication::Person>,
    // `contributors` in Atom, not present in RSS (produces an empty Vec)
    pub contributors: Vec<atom_syndication::Person>,
    // `entries` in Atom, and `items` in RSS
    pub entries: Vec<Entry>,
}

enum FeedData {
    Atom(atom_syndication::Feed),
    RSS(rss::Channel),
}

impl FromStr for FeedData {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<atom_syndication::Feed>() {
            Ok (feed) => Ok (FeedData::Atom(feed)),
            _ => match s.parse::<rss::Rss>() {
                Ok (rss::Rss(channel)) => Ok (FeedData::RSS(channel)),
                _ => Err ("Could not parse XML as Atom or RSS from input")
            }
        }
    }
}

impl ToString for FeedData {
    fn to_string(&self) -> String {
        match self {
            &FeedData::Atom(ref atom_feed) => atom_feed.to_string(),
            &FeedData::RSS(ref rss_channel) => rss::Rss(rss_channel.clone()).to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate atom_syndication;
    extern crate rss;

    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;

    use super::FeedData;

    // Source: https://github.com/vtduncan/rust-atom/blob/master/src/lib.rs
    #[test]
    fn test_from_atom_file() {
        let mut file = File::open("test-data/atom.xml").unwrap();
        let mut atom_string = String::new();
        file.read_to_string(&mut atom_string).unwrap();
        let feed = FeedData::from_str(&atom_string).unwrap();
        assert!(feed.to_string().len() > 0);
    }

    // Source: https://github.com/frewsxcv/rust-rss/blob/master/src/lib.rs
    #[test]
    fn test_from_rss_file() {
        let mut file = File::open("test-data/rss.xml").unwrap();
        let mut rss_string = String::new();
        file.read_to_string(&mut rss_string).unwrap();
        let rss = FeedData::from_str(&rss_string).unwrap();
        assert!(rss.to_string().len() > 0);
    }

    // Source: https://github.com/vtduncan/rust-atom/blob/master/src/lib.rs
    #[test]
    fn test_atom_to_string() {
        let author = atom_syndication::Person {
            name: "N. Blogger".to_string(),
            ..Default::default()
        };

        let entry = atom_syndication::Entry {
            title: "My first post!".to_string(),
            content: Some("This is my first post".to_string()),
            ..Default::default()
        };

        let feed = FeedData::Atom(atom_syndication::Feed {
            title: "My Blog".to_string(),
            authors: vec![author],
            entries: vec![entry],
            ..Default::default()
        });

        assert_eq!(feed.to_string(), "<?xml version=\"1.0\" encoding=\"utf-8\"?><feed xmlns=\'http://www.w3.org/2005/Atom\'><id></id><title>My Blog</title><updated></updated><author><name>N. Blogger</name></author><entry><id></id><title>My first post!</title><updated></updated><content>This is my first post</content></entry></feed>");
    }

    // Source: https://github.com/frewsxcv/rust-rss/blob/master/src/lib.rs
    #[test]
    fn test_rss_to_string() {
        let item = rss::Item {
            title: Some("My first post!".to_string()),
            link: Some("http://myblog.com/post1".to_string()),
            description: Some("This is my first post".to_string()),
            ..Default::default()
        };

        let channel = rss::Channel {
            title: "My Blog".to_string(),
            link: "http://myblog.com".to_string(),
            description: "Where I write stuff".to_string(),
            items: vec![item],
            ..Default::default()
        };

        let rss = FeedData::RSS(channel);
        assert_eq!(rss.to_string(), "<?xml version=\'1.0\' encoding=\'UTF-8\'?><rss version=\'2.0\'><channel><title>My Blog</title><link>http://myblog.com</link><description>Where I write stuff</description><item><title>My first post!</title><link>http://myblog.com/post1</link><description>This is my first post</description></item></channel></rss>");
    }
}
