use rss;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Guid {
    pub is_permalink: bool,
    pub id: String,
}

impl Guid {
    pub fn from_id(id: String) -> Guid {
        Guid {
            is_permalink: true,
            id: id,
        }
    }
}

impl From<rss::Guid> for Guid {
    fn from(id: rss::Guid) -> Guid {
        Guid {
            is_permalink: id.is_perma_link,
            id: id.value,
        }
    }
}

impl From<Guid> for rss::Guid {
    fn from(id: Guid) -> rss::Guid {
        rss::Guid {
            is_perma_link: id.is_permalink,
            value: id.id,
        }
    }
}
