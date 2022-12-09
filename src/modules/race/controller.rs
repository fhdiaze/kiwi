use super::{find, get};
use crate::infra::{db::traits::DynDbClient, api::response};
use axum::{
    extract::{Query, State},
    response::Response,
    routing::get,
    Router,
};

async fn handle_get(State(db): State<DynDbClient>, race_id: String) -> Response {
    let query = get::Query::new(race_id);
    let result = get::handle(db, query).await;

    response::from_result(result)
}

async fn handle_find(State(db): State<DynDbClient>, Query(query): Query<find::Query>) -> Response {
    let query = find::Query::new(query.name, query.city, query.country);
    let result = find::handle(db, query).await;

    response::from_result(result)
}

pub fn route() -> Router<DynDbClient> {
    Router::new()
        .route("/race.get", get(handle_get))
        .route("/race.find", get(handle_find))
}
