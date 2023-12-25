use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Redirect,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::Postgres, FromRow, PgPool, Pool};


pub async fn create_routes(db: Pool<Postgres>) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/shorten/:url", get(shorten))
        .route("/:id", get(redirect))
        .layer(Extension(db));

    Ok(router)
}

pub async fn hello_world() -> String {
    "Hello world axum!".to_owned()
}

#[derive(Deserialize, Serialize, FromRow)]
struct StoredURL {
    pub id: String,
    pub long_url: String,
    pub short_url: String,
}

pub async fn shorten(
    Path(url): Path<String>,
    Extension(conn): Extension<PgPool>,
) -> Result<String, (StatusCode, String)> {
    let id = &nanoid::nanoid!(6);
    println!("ID: {}", id);

    // TODO: Check if the ID already exists in the database
    // If it does, generate a new one and check again
    // If it doesn't, insert the URL into the database

    sqlx::query("INSERT INTO url_mapping VALUES ($1, $2)")
        .bind(id)
        .bind(url)
        .execute(&conn)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to insert URL into database".to_owned(),
            )
        })?;

    Ok(id.to_owned())
}

pub async fn redirect(
    Path(id): Path<String>,
    Extension(conn): Extension<PgPool>,
) -> Result<Redirect, (StatusCode, String)> {
    let result: StoredURL = sqlx::query_as("SELECT url FROM url_mapping WHERE id = $1")
        .bind(id)
        .fetch_one(&conn)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to fetch URL from database".to_owned(),
            )
        })?;

    Ok(Redirect::to(&result.long_url))
}
