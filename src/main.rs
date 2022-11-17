#![deny(nonstandard_style)]
use crate::modules::race;
use axum::Router;

mod domain;
mod modules;

fn route() -> Router {
    Router::new()
        .nest("/api", race::controller::route())
}

#[tokio::main]
async fn main() {
    let router = route();

    axum::Server::bind(&"127.0.0.1:7878".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap()
}
