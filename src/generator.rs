use atom_syndication as atom;

pub struct Generator {
    pub name: String,
    pub uri: Option<String>,
    pub version: Option<String>,
}

impl Generator {
    pub fn from_name(name: String) -> Generator {
        Generator {
            name: name,
            uri: None,
            version: None,
        }
    }
}

impl From<atom::Generator> for Generator {
    fn from(generator: atom::Generator) -> Generator {
        Generator {
            name: generator.name,
            uri: generator.uri,
            version: generator.version,
        }
    }
}

impl Into<atom::Generator> for Generator {
    fn into(self) -> atom::Generator {
        atom::Generator {
            name: self.name,
            uri: self.uri,
            version: self.version,
        }
    }
}
