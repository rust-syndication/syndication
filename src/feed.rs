use atom_syndication as atom;
use rss;

use std::str::FromStr;
use chrono::{DateTime, UTC};
use link::Link;
use person::Person;
use generator::Generator;

use category::Category;
use entry::Entry;

enum FeedData {
    Atom(atom::Feed),
    RSS(rss::Channel),
}

// A helpful table of approximately equivalent elements can be found here:
// http://www.intertwingly.net/wiki/pie/Rss20AndAtom10Compared#table
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
    // `icon` in Atom, not present in RSS
    pub icon: Option<String>,
    // `logo` in Atom, and `image` in RSS
    pub image: Option<String>,

    // `generator` in both Atom and RSS
    pub generator: Option<Generator>,
    // `links` in Atom, and `link` in RSS (produces a 1 item Vec)
    pub links: Vec<Link>,
    // `categories` in both Atom and RSS
    pub categories: Vec<Category>,
    // TODO: Should the `web_master` be in `contributors`, `authors`, or at all?
    // `authors` in Atom, `managing_editor` in RSS (produces 1 item Vec)
    pub authors: Vec<Person>,
    // `contributors` in Atom, `web_master` in RSS (produces a 1 item Vec)
    pub contributors: Vec<Person>,
    // `entries` in Atom, and `items` in RSS
    // TODO: Add more fields that are necessary for RSS
    // TODO: Fancy translation, e.g. Atom <link rel="via"> = RSS `source`
    pub entries: Vec<Entry>,
}

impl From<atom::Feed> for Feed {
    fn from(feed: atom::Feed) -> Self {
        Feed {
            source_data: Some(FeedData::Atom(feed.clone())),
            id: Some(feed.id),
            title: feed.title,
            description: feed.subtitle,
            updated: DateTime::parse_from_rfc3339(feed.updated.as_str())
                .ok()
                .map(|date| date.with_timezone(&UTC)),
            copyright: feed.rights,
            icon: feed.icon,
            image: feed.logo,
            generator: feed.generator.map(|generator| generator.into()),
            links: feed.links.into_iter().map(|link| link.into()).collect(),
            categories: feed.categories.into_iter().map(|person| person.into()).collect(),
            authors: feed.authors.into_iter().map(|person| person.into()).collect(),
            contributors: feed.contributors.into_iter().map(|person| person.into()).collect(),
            entries: feed.entries
                .into_iter()
                .map(|entry| entry.into())
                .collect::<Vec<_>>(),
        }
    }
}

impl From<Feed> for atom::Feed {
    fn from(feed: Feed) -> Self {
        // Performing no translation at all is both faster, and won't lose any data!
        if let Some(FeedData::Atom(feed)) = feed.source_data {
            feed
        } else {
            atom::Feed {
                // TODO: Producing an empty string is probably very very bad
                // is there anything better that can be done...?
                id: feed.id.unwrap_or_else(|| String::from("")),
                title: feed.title,
                subtitle: feed.description,
                // TODO: Is there a better way to handle a missing date here?
                updated: feed.updated.unwrap_or_else(UTC::now).to_rfc3339(),
                rights: feed.copyright,
                icon: feed.icon,
                logo: feed.image,
                generator: None,
                links: feed.links.into_iter().map(|link| link.into()).collect(),
                categories: feed.categories.into_iter().map(|category| category.into()).collect(),
                authors: feed.authors.into_iter().map(|person| person.into()).collect(),
                contributors: feed.contributors.into_iter().map(|person| person.into()).collect(),
                entries: feed.entries
                    .into_iter()
                    .map(|entry| entry.into())
                    .collect::<Vec<_>>(),
            }
        }
    }
}

impl FromStr for Feed {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<FeedData>() {
            Ok(FeedData::Atom(feed)) => Ok(feed.into()),
            // TODO: Implement the RSS conversions
            Ok(FeedData::RSS(_)) => Err("RSS Unimplemented"),
            Err(e) => Err(e),
        }
    }
}

impl FromStr for FeedData {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<atom::Feed>() {
            Ok(feed) => Ok(FeedData::Atom(feed)),
            _ => {
                match s.parse::<rss::Rss>() {
                    Ok(rss::Rss(channel)) => Ok(FeedData::RSS(channel)),
                    _ => Err("Could not parse XML as Atom or RSS from input"),
                }
            }
        }
    }
}

impl ToString for FeedData {
    fn to_string(&self) -> String {
        match *self {
            FeedData::Atom(ref atom_feed) => atom_feed.to_string(),
            FeedData::RSS(ref rss_channel) => rss::Rss(rss_channel.clone()).to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use atom_syndication as atom;
    use rss;

    use std::fs::File;
    use std::io::Read;
    use std::str::FromStr;

    use feed::FeedData;

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
        let author = atom::Person { name: "N. Blogger".to_string(), ..Default::default() };

        let entry = atom::Entry {
            title: "My first post!".to_string(),
            content: Some("This is my first post".to_string()),
            ..Default::default()
        };

        let feed = FeedData::Atom(atom::Feed {
            title: "My Blog".to_string(),
            authors: vec![author],
            entries: vec![entry],
            ..Default::default()
        });

        assert_eq!(feed.to_string(),
                   "<?xml version=\"1.0\" encoding=\"utf-8\"?><feed \
                    xmlns=\'http://www.w3.org/2005/Atom\'><id></id><title>My \
                    Blog</title><updated></updated><author><name>N. \
                    Blogger</name></author><entry><id></id><title>My first \
                    post!</title><updated></updated><content>This is my first \
                    post</content></entry></feed>");
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
        assert_eq!(rss.to_string(),
                   "<?xml version=\'1.0\' encoding=\'UTF-8\'?><rss \
                    version=\'2.0\'><channel><title>My \
                    Blog</title><link>http://myblog.com</link><description>Where I write \
                    stuff</description><item><title>My first \
                    post!</title><link>http://myblog.com/post1</link><description>This is my \
                    first post</description></item></channel></rss>");
    }
}
