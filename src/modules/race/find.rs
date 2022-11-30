use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};

use crate::domain::race::Race;
use crate::infra::config::Config;
use crate::infra::db;
use crate::infra::errors::error::Result;
use crate::modules::common::page::Page;

#[derive(Deserialize)]
pub struct Query {
    pub name: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
}

impl Query {
    pub fn new(name: Option<String>, city: Option<String>, country: Option<String>) -> Self {
        Query {
            name,
            city,
            country,
        }
    }
}

pub async fn handle(query: Query) -> Result<Page<RaceVm>> {
    let races = find_races(query).await?;
    let races_vm: Vec<RaceVm> = races.into_iter().map(|r| RaceVm::new(&r)).collect();
    let page_size = races_vm.len();

    Ok(Page::new(races_vm, 1, page_size, 200))
}

async fn find_races(_: Query) -> Result<Vec<Race>> {
    let config = Config::new();
    let client = db::client::Client::new(&config.db).await?;
    let filter = doc! {};
    let races: Vec<Race> = client.races.find(filter, None).await?.try_collect().await?;

    Ok(races)
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
