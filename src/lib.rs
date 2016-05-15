extern crate atom_syndication;
extern crate rss;
extern crate chrono;

use std::str::FromStr;
use chrono::{DateTime, UTC};

enum EntryData {
    Atom(atom_syndication::Entry),
    RSS(rss::Item),
}

pub struct Category { }
pub struct Link { href: String }
pub struct Person { }

pub struct Entry {
    // If created from an Atom or RSS entry, this is the original contents
    source_data: Option<EntryData>,

    // `id` in Atom (required), and `guid` in RSS
    pub id: Option<String>,
    // `title` in Atom and RSS, optional only in RSS
    pub title: Option<String>,
    // `updated` in Atom (required), not present in RSS
    pub updated: DateTime<UTC>,
    // `published` in Atom, and `pub_date` in RSS
    pub published: Option<DateTime<UTC>>,
    // `summary` in Atom, and `description` in RSS
    pub summary: Option<String>,
    // `content` in Atom, not present in RSS
    pub content: Option<String>,

    // TODO: Figure out the `source` field in the Atom Entry type (It refers to
    // the atom Feed type, which owns the Entry, is it a copy of the Feed with
    // no entries?) How do we include this?

    // `links` in Atom, and `link` in RSS (produces a Vec with 0 or 1 items)
    pub links: Vec<Link>,
    // `categories` in both Atom and RSS
    pub categories: Vec<Category>,
    // `authors` in Atom, `author` in RSS (produces a Vec with 0 or 1 items)
    // TODO: Define our own Person type for API stability reasons
    pub authors: Vec<atom_syndication::Person>,
    // `contributors` in Atom, not present in RSS (produces an empty Vec)
    pub contributors: Vec<atom_syndication::Person>,

    // TODO: What is the RSS `comments` field used for?
}

impl From<atom_syndication::Entry> for Entry {
    fn from(entry: atom_syndication::Entry) -> Self {
        Entry {
            // TODO: We can't move the entry, because we need it's contents
            // and none of the atom_syndication types support .clone() ...
            source_data: None,
            id: Some(entry.id),
            title: Some(entry.title),
            updated: DateTime::parse_from_rfc3339(entry.updated.as_str())
                .map(|date| date.with_timezone(&UTC)).unwrap_or(UTC::now()),
            published: entry.published
                .and_then(|d| DateTime::parse_from_rfc3339(d.as_str()).ok())
                .map(|date| date.with_timezone(&UTC)),
            summary: entry.summary,
            content: entry.content,
            links: entry.links.into_iter()
                .map(|link| Link { href: link.href })
                .collect::<Vec<_>>(),
            // TODO: Implement the Category type for converting this
            categories: vec![],
            authors: entry.authors,
            contributors: entry.contributors,
        }
    }
}

impl From<Entry> for atom_syndication::Entry {
    fn from(entry: Entry) -> Self {
        if let Some(EntryData::Atom(entry)) = entry.source_data {
            entry
        } else {
            atom_syndication::Entry {
                // TODO: How should we handle a missing id?
                id: entry.id.unwrap_or(String::from("")),
                // TODO: How should we handle a missing title?
                title: entry.title.unwrap_or(String::from("")),
                updated: entry.updated.to_rfc3339(),
                published: entry.published.map(|date| date.to_rfc3339()),
                source: None,
                summary: entry.summary,
                content: entry.content,
                links: entry.links.into_iter()
                    .map(|link| atom_syndication::Link {
                        href: link.href, ..Default::default()
                    }).collect::<Vec<_>>(),
                // TODO: Convert from the category type
                categories: vec![],
                authors: entry.authors,
                contributors: entry.contributors,
            }
        }
    }
}

enum FeedData {
    Atom(atom_syndication::Feed),
    RSS(rss::Channel),
}

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
    // `icon` in Atom,
    pub icon: Option<String>,

    // NOTE: Throwing away the `image` field in Atom
    // `generator` in both Atom and RSS
    // TODO: Add a Generator type so this can be implemented
    // pub generator: Option<Generator>,

    // `links` in Atom, and `link` in RSS (will produce a Vec of 1 item)
    pub links: Vec<Link>,
    // `categories` in both Atom and RSS
    pub categories: Vec<Category>,
    // TODO: Define our own Person type for API stability reasons
    // `authors` in Atom, not present in RSS (RSS will produce an empty Vec)
    pub authors: Vec<atom_syndication::Person>,
    // `contributors` in Atom, not present in RSS (produces an empty Vec)
    pub contributors: Vec<atom_syndication::Person>,
    // `entries` in Atom, and `items` in RSS
    pub entries: Vec<Entry>,
}

impl From<atom_syndication::Feed> for Feed {
    fn from(feed: atom_syndication::Feed) -> Self {
        Feed {
            // TODO: We can't move the feed, because we need its contents
            // and none of the atom_syndication types support .clone() ...
            source_data: None, // Some(FeedData::Atom(feed.clone())),
            id: Some(feed.id),
            title: feed.title,
            description: feed.subtitle,
            updated: DateTime::parse_from_rfc3339(feed.updated.as_str()).ok()
                .map(|date| date.with_timezone(&UTC)),
            copyright: feed.rights,
            icon: feed.icon,
            // NOTE: We throw away the `image` field
            // NOTE: We throw away the generator field
            // TODO: Add more fields to the link type
            links: feed.links.into_iter()
                .map(|link| Link { href: link.href })
                .collect::<Vec<_>>(),
            // TODO: Handle this once the Category type is defined
            categories: vec![],
            authors: feed.authors,
            contributors: feed.contributors,
            entries: feed.entries.into_iter().map(|entry| entry.into())
                .collect::<Vec<_>>(),
        }
    }
}

impl From<Feed> for atom_syndication::Feed {
    fn from(feed: Feed) -> Self {
        // Performing no translation at all is both faster, and won't lose any data!
        if let Some(FeedData::Atom(feed)) = feed.source_data {
            feed
        } else {
            atom_syndication::Feed {
                // TODO: Producing an empty string is probably very very bad
                // is there anything better that can be done...?
                id: feed.id.unwrap_or(String::from("")),
                title: feed.title,
                subtitle: feed.description,
                // TODO: Is there a better way to handle a missing date here?
                updated: feed.updated.unwrap_or(UTC::now()).to_rfc3339(),
                rights: feed.copyright,
                icon: feed.icon.clone(),
                logo: feed.icon,
                generator: None,
                links: feed.links.into_iter()
                    .map(|link| atom_syndication::Link {
                        href: link.href, ..Default::default()
                    }).collect::<Vec<_>>(),
                // TODO: Convert from our Category type instead of throwing them away
                categories: vec![],
                authors: feed.authors,
                contributors: feed.contributors,
                entries: feed.entries.into_iter().map(|entry| entry.into())
                    .collect::<Vec<_>>(),
            }
        }
    }
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
