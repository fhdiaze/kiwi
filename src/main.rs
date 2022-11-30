#![deny(nonstandard_style)]
use std::sync::Arc;

use crate::modules::race;
use axum::Router;
use infra::{db, config};

mod domain;
mod modules;
mod infra;

type DynDbClient = Arc<dyn db::traits::Client>;

fn route() -> Router {
    Router::new()
        .nest("/api", race::controller::route())
}

#[tokio::main]
async fn main() {
    let router = route();

    let cfg = config::Config::new();
    let db_client = Arc::new(db::client::Client::new(&cfg.db).await.unwrap()) as DynDbClient;
    router.with_state(db_client);

    axum::Server::bind(&"127.0.0.1:7878".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}

