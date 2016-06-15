use atom_syndication as atom;
use rss;

use std::str::FromStr;
use std::fmt::{Debug, Formatter, Error};
use chrono::{DateTime, UTC};

use link::Link;
use person::Person;
use generator::Generator;
use category::Category;
use entry::Entry;
use image::Image;
use text_input::TextInput;

#[derive(Clone)]
enum FeedData {
    Atom(atom::Feed),
    Rss(rss::Channel),
}

impl Debug for FeedData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            FeedData::Atom(_) => write!(f, "Atom(_)"),
            FeedData::Rss(_) => write!(f, "Rss(_)"),
        }
    }
}

// A helpful table of approximately equivalent elements can be found here:
// http://www.intertwingly.net/wiki/pie/Rss20AndAtom10Compared#table
#[derive(Debug, Clone)]
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
    pub image: Option<Image>,

    // `generator` in both Atom and RSS
    pub generator: Option<Generator>,
    // `links` in Atom, and `link` in RSS (produces a 1 item Vec)
    pub links: Vec<Link>,
    // `categories` in both Atom and RSS
    pub categories: Vec<Category>,
    // `authors` in Atom, `managing_editor` in RSS (produces 1 item Vec)
    // TODO: Should the `web_master` be in `contributors`, `authors`, or at all?
    pub authors: Vec<Person>,
    // `contributors` in Atom, `web_master` in RSS (produces a 1 item Vec)
    pub contributors: Vec<Person>,
    // `entries` in Atom, and `items` in RSS
    // TODO: Fancy translation, e.g. Atom <link rel="via"> = RSS `source`
    pub entries: Vec<Entry>,

    // TODO: Add more fields that are necessary for RSS
    // `ttl` in RSS, not present in Atom
    pub ttl: Option<String>,
    // `skip_hours` in RSS, not present in Atom
    pub skip_hours: Option<String>,
    // `skip_days` in RSS, not present in Atom
    pub skip_days: Option<String>,
    // `text_input` in RSS, not present in Atom
    pub text_input: Option<TextInput>,
    // `language` in RSS, not present in Atom
    pub language: Option<String>,
    // `docs` in RSS, not present in Atom
    pub docs: Option<String>,
    // `rating` in RSS, not present in Atom
    pub rating: Option<String>,
}

impl Feed {
    pub fn to_rss_string(&self) -> String {
        if let Some(FeedData::Rss(ref feed)) = self.source_data {
            rss::Rss(feed.clone()).to_string()
        } else {
            let rss: rss::Channel = self.clone().into();
            rss::Rss(rss).to_string()
        }
    }

    pub fn to_atom_string(&self) -> String {
        if let Some(FeedData::Atom(ref feed)) = self.source_data {
            feed.to_string()
        } else {
            let atom: atom::Feed = self.clone().into();
            atom.to_string()
        }
    }
}

impl From<atom::Feed> for Feed {
    fn from(feed: atom::Feed) -> Self {
        let feed_clone = feed.clone();
        let title = feed.title.clone();
        let link = feed.links.first().map_or_else(|| "".into(), |link| link.href.clone());
        Feed {
            source_data: Some(FeedData::Atom(feed_clone)),
            id: Some(feed.id),
            title: feed.title,
            description: feed.subtitle,
            updated: DateTime::parse_from_rfc3339(feed.updated.as_str())
                .ok()
                .map(|date| date.with_timezone(&UTC)),
            copyright: feed.rights,
            icon: feed.icon,
            // (Note, in practice the image <title> and <link> should have the same value as the
            // channel's <title> and <link>.)
            image: feed.logo.map(|url| {
                Image {
                    url: url,
                    title: title,
                    link: link,
                    width: None,
                    height: None,
                }
            }),
            generator: feed.generator.map(|generator| generator.into()),
            links: feed.links.into_iter().map(|link| link.into()).collect(),
            categories: feed.categories.into_iter().map(|person| person.into()).collect(),
            authors: feed.authors.into_iter().map(|person| person.into()).collect(),
            contributors: feed.contributors.into_iter().map(|person| person.into()).collect(),
            entries: feed.entries
                .into_iter()
                .map(|entry| entry.into())
                .collect::<Vec<_>>(),
            ttl: None,
            skip_hours: None,
            skip_days: None,
            text_input: None,
            language: None,
            docs: None,
            rating: None,
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
                logo: feed.image.map(|image| image.url),
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

impl From<rss::Channel> for Feed {
    fn from(feed: rss::Channel) -> Self {
        Feed {
            source_data: Some(FeedData::Rss(feed.clone())),
            id: None,
            title: feed.title,
            description: Some(feed.description),
            updated: None,
            copyright: feed.copyright,
            icon: None,
            image: feed.image.map(|image| image.into()),
            generator: feed.generator.map(Generator::from_name),
            links: vec![Link::from_href(feed.link)],
            categories: feed.categories.into_iter().map(|person| person.into()).collect(),
            authors: feed.managing_editor.into_iter().map(Person::from_name).collect(),
            contributors: feed.web_master.into_iter().map(Person::from_name).collect(),
            entries: feed.items.into_iter().map(|entry| entry.into()).collect(),
            ttl: feed.ttl,
            skip_hours: feed.skip_hours,
            skip_days: feed.skip_days,
            text_input: feed.text_input.map(|input| input.into()),
            rating: feed.rating,
            language: feed.language,
            docs: feed.docs,
        }
    }
}

impl From<Feed> for rss::Channel {
    fn from(feed: Feed) -> rss::Channel {
        if let Some(FeedData::Rss(feed)) = feed.source_data {
            feed
        } else {
            rss::Channel {
                title: feed.title,
                description: feed.description.unwrap_or("".into()),
                pub_date: None,
                last_build_date: feed.updated.map(|date| date.to_rfc2822()),
                link: feed.links.into_iter().next().map_or_else(|| "".into(), |link| link.href),
                items: feed.entries.into_iter().map(|entry| entry.into()).collect(),
                categories: feed.categories.into_iter().map(|category| category.into()).collect(),
                image: feed.image.map(|image| image.into()),
                generator: feed.generator.map(|generator| generator.name),
                managing_editor: feed.authors.into_iter().next().map(|person| person.name),
                web_master: feed.contributors.into_iter().next().map(|person| person.name),
                copyright: feed.copyright,
                ttl: feed.ttl,
                skip_hours: feed.skip_hours,
                skip_days: feed.skip_days,
                text_input: feed.text_input.map(|input| input.into()),
                rating: feed.rating,
                language: feed.language,
                docs: feed.docs,
            }
        }
    }
}

impl FromStr for Feed {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<FeedData>() {
            Ok(FeedData::Atom(feed)) => Ok(feed.into()),
            Ok(FeedData::Rss(feed)) => Ok(feed.into()),
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
                    Ok(rss::Rss(channel)) => Ok(FeedData::Rss(channel)),
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
            FeedData::Rss(ref rss_channel) => rss::Rss(rss_channel.clone()).to_string(),
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

    use super::{FeedData, Feed};

    // Source: https://github.com/vtduncan/rust-atom/blob/master/src/lib.rs
    #[test]
    fn test_from_atom_file() {
        let mut file = File::open("test-data/atom.xml").unwrap();
        let mut atom_string = String::new();
        file.read_to_string(&mut atom_string).unwrap();
        let feed = FeedData::from_str(&atom_string).unwrap();
        // TODO: Assert a stronger property on this
        assert!(feed.to_string().len() > 0);
    }

    #[test]
    fn test_feed_from_atom_file() {
        let mut file = File::open("test-data/atom.xml").unwrap();
        let mut atom_string = String::new();
        file.read_to_string(&mut atom_string).unwrap();
        let feed = Feed::from_str(&atom_string).unwrap();
        // TODO: Assert a stronger property than this
        assert!(feed.to_atom_string().len() > 0);
    }

    // Source: https://github.com/frewsxcv/rust-rss/blob/master/src/lib.rs
    #[test]
    fn test_from_rss_file() {
        let mut file = File::open("test-data/rss.xml").unwrap();
        let mut rss_string = String::new();
        file.read_to_string(&mut rss_string).unwrap();
        let rss = FeedData::from_str(&rss_string).unwrap();
        // TODO: Assert a stronger property than this
        assert!(rss.to_string().len() > 0);
    }

    #[test]
    fn test_feed_from_rss_file() {
        let mut file = File::open("test-data/rss.xml").unwrap();
        let mut rss_string = String::new();
        file.read_to_string(&mut rss_string).unwrap();
        let rss = Feed::from_str(&rss_string).unwrap();
        // TODO: Assert a stronger property than this
        assert!(rss.to_rss_string().len() > 0);
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

        let rss = FeedData::Rss(channel);
        assert_eq!(rss.to_string(),
                   "<?xml version=\'1.0\' encoding=\'UTF-8\'?><rss \
                    version=\'2.0\'><channel><title>My \
                    Blog</title><link>http://myblog.com</link><description>Where I write \
                    stuff</description><item><title>My first \
                    post!</title><link>http://myblog.com/post1</link><description>This is my \
                    first post</description></item></channel></rss>");
    }
}
