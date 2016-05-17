extern crate atom_syndication;
extern crate rss;
extern crate chrono;

use chrono::{DateTime, UTC};
use category::Category;
use link::Link;

enum EntryData {
    Atom(atom_syndication::Entry),
    RSS(rss::Item),
}

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
    // `summary` in Atom
    pub summary: Option<String>,
    // `content` in Atom, `description` in RSS
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
    // TODO: Define our own Person type for API stability reasons
    pub authors: Vec<atom_syndication::Person>,
    // `contributors` in Atom, not present in RSS (produces an empty Vec)
    pub contributors: Vec<atom_syndication::Person>,
}

impl From<atom_syndication::Entry> for Entry {
    fn from(entry: atom_syndication::Entry) -> Self {
        Entry {
            source_data: Some(EntryData::Atom(entry.clone())),
            id: Some(entry.id),
            title: Some(entry.title),
            updated: DateTime::parse_from_rfc3339(entry.updated.as_str())
                .map(|date| date.with_timezone(&UTC))
                .unwrap_or(UTC::now()),
            published: entry.published
                .and_then(|d| DateTime::parse_from_rfc3339(d.as_str()).ok())
                .map(|date| date.with_timezone(&UTC)),
            summary: entry.summary,
            content: entry.content,
            links: entry.links
                .into_iter()
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
                title: entry.title.unwrap_or(String::from("")),
                updated: entry.updated.to_rfc3339(),
                published: entry.published.map(|date| date.to_rfc3339()),
                source: None,
                summary: entry.summary,
                content: entry.content,
                links: entry.links
                    .into_iter()
                    .map(|link| atom_syndication::Link { href: link.href, ..Default::default() })
                    .collect::<Vec<_>>(),
                // TODO: Convert from the category type
                categories: vec![],
                authors: entry.authors,
                contributors: entry.contributors,
            }
        }
    }
}
