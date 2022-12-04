use super::{find, get};
use crate::{modules::common::page::Page, infra::db::traits::DynDbClient};
use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};

async fn handle_get(State(db): State<DynDbClient>, race_id: String) -> Json<get::RaceVm> {
    let query = get::Query::new(race_id);
    let race_vm = get::handle(db, query).await.unwrap();

    Json(race_vm)
}

async fn handle_find(
    State(db): State<DynDbClient>,
    Query(query): Query<find::Query>,
) -> Json<Page<find::RaceVm>> {
    let query = find::Query::new(query.name, query.city, query.country);
    let races = find::handle(db, query).await.unwrap();

    Json(races)
}

pub fn route() -> axum::Router<DynDbClient> {
    Router::new()
        .route("/race.get", get(handle_get))
        .route("/race.find", get(handle_find))
}
