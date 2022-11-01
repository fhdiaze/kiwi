use crate::domain::{discipline::Discipline, location::Location, race::Race};

pub struct Query {
    id: usize,
}

impl Query {
    pub fn new(id: usize) -> Self {
        Query { id }
    }
}

pub fn handle(query: Query) -> Race {
    Race::new(
        query.id,
        "Go Rigo Go".to_string(),
        120.5,
        Discipline::Road,
        Location {
            address: String::from("Centro"),
            city: String::from("Villavicencio"),
            state: String::from("Meta"),
            country: String::from("Colombia"),
        },
    )
}
