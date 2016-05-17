use atom_syndication as atom;

pub struct Person {
    pub name: String,
    pub uri: Option<String>,
    pub email: Option<String>,
}

impl Person {
    pub fn from_name(name: String) -> Person {
        Person {
            name: name,
            uri: None,
            email: None,
        }
    }
}

impl From<atom::Person> for Person {
    fn from(person: atom::Person) -> Person {
        Person {
            name: person.name,
            uri: person.uri,
            email: person.email,
        }
    }
}

impl Into<atom::Person> for Person {
    fn into(self) -> atom::Person {
        atom::Person {
            name: self.name,
            uri: self.uri,
            email: self.email,
        }
    }
}
