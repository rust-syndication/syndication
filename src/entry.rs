use atom_syndication as atom;
use rss;

use std::str::FromStr;
use std::fmt::{Formatter, Debug, Error};
use chrono::{DateTime, UTC};

use category::Category;
use link::Link;
use person::Person;
use guid::Guid;

#[derive(Clone)]
enum EntryData {
    Atom(atom::Entry),
    Rss(rss::Item),
}

impl Debug for EntryData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            EntryData::Atom(_) => write!(f, "Atom(_)"),
            EntryData::Rss(_) => write!(f, "Rss(_)"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    // If created from an Atom or RSS entry, this is the original contents
    source_data: Option<EntryData>,

    // `id` in Atom (required), and `guid` in RSS
    pub id: Option<Guid>,
    // `title` in Atom and RSS, optional only in RSS
    pub title: Option<String>,
    // `updated` in Atom (required), not present in RSS
    pub updated: DateTime<UTC>,
    // `published` in Atom, and `pub_date` in RSS
    pub published: Option<DateTime<UTC>>,
    // `summary` in Atom
    pub summary: Option<String>,
    // `content` in Atom, `description` in RSS
    // TODO: Change this to include the type information from atom_syndication
    pub content: Option<String>,

    // TODO: Figure out the `source` field in the Atom Entry type (It refers to
    // the atom Feed type, which owns the Entry, is it a copy of the Feed with
    // no entries?) How do we include this?
    //
    // `links` in Atom, and `link` in RSS (produces a Vec with 0 or 1 items)
    pub links: Vec<Link>,
    // `categories` in both Atom and RSS
    pub categories: Vec<Category>,
    // `authors` in Atom, `author` in RSS (produces a Vec with 0 or 1 items)
    pub authors: Vec<Person>,
    // `contributors` in Atom, not present in RSS (produces an empty Vec)
    pub contributors: Vec<Person>,
    // not present in Atom (produces None), `comments` in RSS
    pub comments: Option<String>,
}

impl From<atom::Entry> for Entry {
    fn from(entry: atom::Entry) -> Self {
        Entry {
            source_data: Some(EntryData::Atom(entry.clone())),
            id: Some(Guid::from_id(entry.id)),
            title: Some(entry.title),
            updated: DateTime::parse_from_rfc3339(entry.updated.as_str())
                .map(|date| date.with_timezone(&UTC))
                .unwrap_or_else(|_| UTC::now()),
            published: entry.published
                .and_then(|d| DateTime::parse_from_rfc3339(d.as_str()).ok())
                .map(|date| date.with_timezone(&UTC)),
            summary: entry.summary,
            content: entry.content.map(|x| match x {
                atom::Content::Text(s) | atom::Content::Html(s) => s,
                atom::Content::Xhtml(x) => x.to_string(),
            }),
            links: entry.links.into_iter().map(|link| link.into()).collect(),
            categories: entry.categories.into_iter().map(|category| category.into()).collect(),
            authors: entry.authors.into_iter().map(|person| person.into()).collect(),
            contributors: entry.contributors.into_iter().map(|person| person.into()).collect(),
            comments: None,
        }
    }
}

impl From<Entry> for atom::Entry {
    fn from(entry: Entry) -> Self {
        if let Some(EntryData::Atom(entry)) = entry.source_data {
            entry
        } else {
            atom::Entry {
                // TODO: How should we handle a missing id?
                id: entry.id.unwrap_or_else(|| Guid::from_id("".into())).id,
                title: entry.title.unwrap_or_else(|| String::from("")),
                updated: entry.updated.to_rfc3339(),
                published: entry.published.map(|date| date.to_rfc3339()),
                // TODO: Figure out this thing...
                source: None,
                summary: entry.summary,
                content: entry.content.map(atom::Content::Text),
                links: entry.links.into_iter().map(|link| link.into()).collect(),
                categories: entry.categories.into_iter().map(|category| category.into()).collect(),
                authors: entry.authors.into_iter().map(|person| person.into()).collect(),
                contributors: entry.contributors.into_iter().map(|person| person.into()).collect(),
            }
        }
    }
}

impl From<rss::Item> for Entry {
    fn from(entry: rss::Item) -> Self {
        let entry_clone = entry.clone();
        let date = entry.pub_date.and_then(|d| DateTime::from_str(&d[..]).ok());
        Entry {
            source_data: Some(EntryData::Rss(entry_clone)),
            id: entry.guid.map(|id| id.into()),
            title: entry.title,
            updated: date.unwrap_or_else(UTC::now),
            published: date,
            summary: None,
            content: entry.description,
            links: entry.link.into_iter().map(Link::from_href).collect(),
            categories: entry.categories.into_iter().map(|category| category.into()).collect(),
            authors: entry.author.into_iter().map(Person::from_name).collect(),
            contributors: vec![],
            comments: entry.comments,
        }
    }
}

impl From<Entry> for rss::Item {
    fn from(entry: Entry) -> rss::Item {
        if let Some(EntryData::Rss(entry)) = entry.source_data {
            entry
        } else {
            rss::Item {
                guid: entry.id.map(|id| id.into()),
                title: entry.title,
                author: entry.authors.into_iter().next().map(|person| person.name),
                pub_date: entry.published.map(|date| date.to_rfc2822()),
                description: entry.content,
                link: entry.links.into_iter().next().map(|link| link.href),
                categories: entry.categories.into_iter().map(|category| category.into()).collect(),
                comments: entry.comments,
            }
        }
    }
}
