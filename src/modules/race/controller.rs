use std::sync::Arc;

use super::{find, get};
use crate::{modules::common::page::Page, infra::db};
use axum::{extract::{Query, State}, http::StatusCode, response::IntoResponse, routing::get, Json, Router};

async fn handle_get() -> Json<get::RaceVm> {
    let query = get::Query::new(String::from("124324"));
    let race_vm = get::handle(query);

    Json(race_vm)
}

async fn handle_find(State(db_client): State<Arc<dyn db::traits::Client>> ,Query(query): Query<find::Query>) -> Json<Page<find::RaceVm>> {
    let query = find::Query::new(query.name, query.city, query.country);
    let races = find::handle(query).await.unwrap();

    Json(races)
}

pub fn route() -> axum::Router {
    Router::new()
        .route("/race.get", get(handle_get))
        .route("/race.find", get(handle_find))
}
