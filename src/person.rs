use atom_syndication as atom;

#[derive(Debug, PartialEq, Clone)]
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

impl From<Person> for atom::Person {
    fn from(person: Person) -> atom::Person {
        atom::Person {
            name: person.name,
            uri: person.uri,
            email: person.email,
        }
    }
}
