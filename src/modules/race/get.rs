use crate::{
    domain::race::Race,
    infra::{core::result::Result, db::traits::DynDbClient, error::AppError},
};
use bson::doc;
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

pub async fn handle(db: DynDbClient, query: Query) -> Result<RaceVm> {
    let filter = doc! {"_id": &query.id};
    let opt_race = db.races().find_one(filter, None).await?;

    match opt_race {
        Some(race) => Ok(RaceVm::new(&race)),
        None => Err(Box::new(AppError::NotFound(format!(
            "No race was found with id={}",
            &query.id
        )))),
    }
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
