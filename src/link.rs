use atom_syndication as atom;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Link {
    pub href: String,
    pub rel: Option<String>,
    pub mediatype: Option<String>,
    pub hreflang: Option<String>,
    pub title: Option<String>,
    pub length: Option<String>,
}

impl Link {
    pub fn from_href(href: String) -> Link {
        Link {
            href: href,
            rel: None,
            mediatype: None,
            hreflang: None,
            title: None,
            length: None,
        }
    }
}

impl From<atom::Link> for Link {
    fn from(link: atom::Link) -> Link {
        Link {
            href: link.href,
            rel: link.rel,
            mediatype: link.mediatype,
            hreflang: link.hreflang,
            title: link.title,
            length: link.length,
        }
    }
}

impl From<Link> for atom::Link {
    fn from(link: Link) -> atom::Link {
        atom::Link {
            href: link.href,
            rel: link.rel,
            mediatype: link.mediatype,
            hreflang: link.hreflang,
            title: link.title,
            length: link.length,
        }
    }
}
