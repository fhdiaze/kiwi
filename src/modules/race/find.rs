use serde::Serialize;

use crate::domain::{discipline::Discipline, location::Location, race::Race};

pub struct Query {
    name: String,
    city: String,
    country: String,
}

impl Query {
    pub fn new(name: String, city: String, country: String) -> Self {
        Query {
            name,
            city,
            country,
        }
    }
}

pub fn handle(_: Query) -> Page<RaceVm> {
    let races = vec![
        Race::new(
            1,
            "Go Rigo Go".to_string(),
            120.5,
            Discipline::Road,
            Location {
                address: String::from("Centro"),
                city: String::from("Villavicencio"),
                state: String::from("Meta"),
                country: String::from("Colombia"),
            },
        ),
        Race::new(
            2,
            "Gfny Bogota".to_string(),
            120.5,
            Discipline::Road,
            Location {
                address: String::from("Centro"),
                city: String::from("Guatavita"),
                state: String::from("Cundinamarca"),
                country: String::from("Colombia"),
            },
        ),
    ];

    let races_vm = races.into_iter().map(|r| RaceVm::new(&r)).collect();

    Page::new(races_vm, races_vm.len(), 200)
}

#[derive(Serialize)]
pub struct RaceVm {
    id: usize,
    name: String,
}

impl RaceVm {
    fn new(race: &Race) -> Self {
        RaceVm {
            id: race.id,
            name: race.name.clone(),
        }
    }
}
