use crate::domain::{discipline::Discipline, location::Location, race::Race};
use serde::Serialize;

pub struct Query {
    /// The id of the race
    id: String,
}

impl Query {
    pub fn new(id: String) -> Self {
        Query { id }
    }
}

pub fn handle(query: Query) -> RaceVm {
    let race = Race::new(
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
    );

    RaceVm::new(&race)
}

#[derive(Serialize)]
pub struct RaceVm {
    id: String,
    name: String,
}

impl RaceVm {
    fn new(race: &Race) -> Self {
        RaceVm {
            id: race.id.clone(),
            name: race.name.clone(),
        }
    }
}
