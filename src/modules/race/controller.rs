use crate::modules::{common::page::Page};
use super::{get, find};
use axum::{Router, routing::get, Json};

async fn handle_get() -> Json<get::RaceVm> {
    let query = get::Query::new(String::from("124324"));
    let race_vm = get::handle(query);

    Json(race_vm)
}

async fn handle_find() -> Json<Page<find::RaceVm>> {
    let query = find::Query::new(String::from(""), String::from(""), String::from(""));
    let races = find::handle(query).await.unwrap();

    Json(races)
}

pub fn route() -> axum::Router {
    Router::new()
        .route("/race.get", get(handle_get))
        .route("/race.find", get(handle_find))
}
