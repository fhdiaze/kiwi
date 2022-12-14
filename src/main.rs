#![deny(nonstandard_style)]
use crate::modules::race;
use axum::Router;
use infra::{
    config,
    db::{self, traits::DynDbClient},
};
use std::sync::Arc;
use tower::ServiceBuilder;

mod domain;
mod infra;
mod modules;

fn route() -> Router<DynDbClient> {
    let races = race::controller::route();
    Router::new().nest("/api", races)
}

#[tokio::main]
async fn main() {
    let cfg = config::Config::new();
    let db = Arc::new(db::client::Client::new(&cfg.db).await.unwrap()) as DynDbClient;
    let router = route().layer(ServiceBuilder::new()).with_state(db);

    axum::Server::bind(&"0.0.0.0:7878".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
