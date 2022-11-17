use crate::modules::race;
use axum::{self, Json};

async fn handle_get() -> Json<race::get::RaceVm> {
    let query = race::get::Query::new(1);
    let race_vm = race::get::handle(query);

    Json(race_vm)
}

async fn handle_find() -> Json<Vec<race::find::RaceVm>> {
    let query = race::find::Query::new(String::from(""), String::from(""), String::from(""));
    let races = race::find::handle(query);

    Json(races)
}

pub fn route() -> axum::Router {
    axum::Router::new()
        .route("/race.get", axum::routing::get(handle_get))
        .route("/race.find", axum::routing::get(handle_find))
}
