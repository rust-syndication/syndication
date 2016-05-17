use atom_syndication as atom;
use rss;

use chrono::{DateTime, UTC};
use category::Category;
use link::Link;
use person::Person;

enum EntryData {
    Atom(atom::Entry),
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
    pub authors: Vec<Person>,
    // `contributors` in Atom, not present in RSS (produces an empty Vec)
    pub contributors: Vec<Person>,
}

impl From<atom::Entry> for Entry {
    fn from(entry: atom::Entry) -> Self {
        Entry {
            source_data: Some(EntryData::Atom(entry.clone())),
            id: Some(entry.id),
            title: Some(entry.title),
            updated: DateTime::parse_from_rfc3339(entry.updated.as_str())
                .map(|date| date.with_timezone(&UTC))
                .unwrap_or_else(|_| UTC::now()),
            published: entry.published
                .and_then(|d| DateTime::parse_from_rfc3339(d.as_str()).ok())
                .map(|date| date.with_timezone(&UTC)),
            summary: entry.summary,
            content: entry.content,
            links: entry.links.into_iter().map(|link| link.into()).collect(),
            categories: entry.categories.into_iter().map(|category| category.into()).collect(),
            authors: entry.authors.into_iter().map(|person| person.into()).collect(),
            contributors: entry.contributors.into_iter().map(|person| person.into()).collect(),
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
                id: entry.id.unwrap_or_else(|| String::from("")),
                title: entry.title.unwrap_or_else(|| String::from("")),
                updated: entry.updated.to_rfc3339(),
                published: entry.published.map(|date| date.to_rfc3339()),
                source: None,
                summary: entry.summary,
                content: entry.content,
                links: entry.links.into_iter().map(|link| link.into()).collect(),
                categories: entry.categories.into_iter().map(|category| category.into()).collect(),
                authors: entry.authors.into_iter().map(|person| person.into()).collect(),
                contributors: entry.contributors.into_iter().map(|person| person.into()).collect(),
            }
        }
    }
}
