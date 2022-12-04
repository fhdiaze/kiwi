#![deny(nonstandard_style)]
use std::sync::Arc;

use crate::modules::race;
use axum::Router;
use infra::{config, db::{self, traits::DynDbClient}};

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
    let router = route().with_state(db);

    axum::Server::bind(&"127.0.0.1:7878".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
