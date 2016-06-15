use atom_syndication as atom;
use rss;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Category {
    pub term: String,
    pub scheme: Option<String>,
    pub label: Option<String>,
}

impl From<atom::Category> for Category {
    fn from(category: atom::Category) -> Category {
        Category {
            term: category.term,
            scheme: category.scheme,
            label: category.label,
        }
    }
}

impl From<Category> for atom::Category {
    fn from(category: Category) -> atom::Category {
        atom::Category {
            term: category.term,
            scheme: category.scheme,
            label: category.label,
        }
    }
}

impl From<rss::Category> for Category {
    fn from(category: rss::Category) -> Category {
        Category {
            term: category.value,
            scheme: category.domain,
            // TODO: Should we duplicate the term in the label?
            label: None,
        }
    }
}

impl From<Category> for rss::Category {
    fn from(category: Category) -> rss::Category {
        rss::Category {
            value: category.term,
            domain: category.scheme,
        }
    }
}
