use serde::Serialize;

use crate::domain::{discipline::Discipline, location::Location, race::Race};
use crate::infra::db;
use crate::infra::db::cosmos::CosmosClient;
use crate::modules::common::page::Page;

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

pub async fn handle(_: Query) -> Page<RaceVm> {
    let client = db::mongo::client();
    let races = client.races.find().await;    
    
    let races_vm: Vec<RaceVm> = races.into_iter().map(|r| RaceVm::new(&r)).collect();
    let page_size = races_vm.len();

    Page::new(races_vm, 1, page_size, 200)
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
