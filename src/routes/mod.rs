use axum::{
    routing::{get, post},
    Router, Extension,
};
use sqlx::{postgres::Postgres, Pool};

use crate::handlers::{hello_world, redirect, shorten};


pub async fn create_routes(db: Pool<Postgres>) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/:id", get(redirect))
        .route("/shorten/:url", post(shorten))
        .layer(Extension(db));

    Ok(router)
}