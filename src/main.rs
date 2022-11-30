#![deny(nonstandard_style)]
use std::sync::Arc;

use crate::modules::race;
use axum::Router;
use infra::{config, db::{self, traits::DynDbClient}};

mod domain;
mod infra;
mod modules;

fn route() -> Router<DynDbClient> {
    Router::new().nest("/api", race::controller::route())
}

#[tokio::main]
async fn main() {
    let cfg = config::Config::new();
    let db_client = Arc::new(db::client::Client::new(&cfg.db).await.unwrap()) as DynDbClient;
    let router = route().with_state(db_client);

    axum::Server::bind(&"127.0.0.1:7878".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
