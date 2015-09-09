extern crate atom_syndication;
extern crate rss;

use std::str::FromStr;

pub enum Feed {
    Atom(atom_syndication::Feed),
    RSS(rss::Channel),
}

impl FromStr for Feed {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<atom_syndication::Feed>() {
            Ok (feed) => Ok (Feed::Atom(feed)),
            _ => match s.parse::<rss::Rss>() {
                Ok (rss::Rss(channel)) => Ok (Feed::RSS(channel)),
                _ => Err ("Could not parse XML as Atom or RSS from input")
            }
        }
    }
}

impl ToString for Feed {
    fn to_string(&self) -> String {
        match self {
            &Feed::Atom(ref atom_feed) => atom_feed.to_string(),
            &Feed::RSS(ref rss_channel) => rss::Rss(rss_channel.clone()).to_string(),
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

    use super::Feed;

    // Source: https://github.com/vtduncan/rust-atom/blob/master/src/lib.rs
    #[test]
    fn test_from_atom_file() {
        let mut file = File::open("test-data/atom.xml").unwrap();
        let mut atom_string = String::new();
        file.read_to_string(&mut atom_string).unwrap();
        let feed = Feed::from_str(&atom_string).unwrap();
        assert!(feed.to_string().len() > 0);
    }

    // Source: https://github.com/frewsxcv/rust-rss/blob/master/src/lib.rs
    #[test]
    fn test_from_rss_file() {
        let mut file = File::open("test-data/rss.xml").unwrap();
        let mut rss_string = String::new();
        file.read_to_string(&mut rss_string).unwrap();
        let rss = Feed::from_str(&rss_string).unwrap();
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

        let feed = Feed::Atom(atom_syndication::Feed {
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

        let rss = Feed::RSS(channel);
        assert_eq!(rss.to_string(), "<?xml version=\'1.0\' encoding=\'UTF-8\'?><rss version=\'2.0\'><channel><title>My Blog</title><link>http://myblog.com</link><description>Where I write stuff</description><item><title>My first post!</title><link>http://myblog.com/post1</link><description>This is my first post</description></item></channel></rss>");
    }
}
