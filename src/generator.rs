use atom_syndication as atom;

#[derive(Debug, PartialEq, Eq, Clone)]
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

impl From<Generator> for atom::Generator {
    fn from(generator: Generator) -> atom::Generator {
        atom::Generator {
            name: generator.name,
            uri: generator.uri,
            version: generator.version,
        }
    }
}
