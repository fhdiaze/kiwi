use super::{find, get};
use crate::infra::{
    core::{page::Page, result::Result},
    db::traits::DynDbClient,
};
use axum::{
    extract::{Query, State},
    routing::get,
    Router,
};

async fn handle_get(
    State(db): State<DynDbClient>,
    Query(query): Query<get::Query>,
) -> Result<get::RaceVm> {
    let race = get::handle(db, query).await?;

    Ok(race)
}

async fn handle_find(
    State(db): State<DynDbClient>,
    Query(query): Query<find::Query>,
) -> Result<Page<find::RaceVm>> {
    let query = find::Query::new(query.name, query.city, query.country);
    let races = find::handle(db, query).await?;

    Ok(races)
}

pub fn route() -> Router<DynDbClient> {
    Router::new()
        .route("/race.get", get(handle_get))
        .route("/race.find", get(handle_find))
}
