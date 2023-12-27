use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Redirect,
    routing::{get, post},
    Router, Json,
};
use sqlx::{postgres::Postgres, PgPool, Pool};

use crate::types::{StoredURL, BASE_URL, AppError};
use crate::services::{check_if_id_exists, shorten_url, get_single_url, get_all_url};


pub async fn create_routes(db: Pool<Postgres>) -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/shorten/:url", post(shorten))
        .route("/:id", get(redirect))
        .layer(Extension(db));

    Ok(router)
}

pub async fn hello_world(
        Extension(conn): Extension<PgPool>,
)  -> Result<Json<Vec<StoredURL>>, AppError> {
    let data= get_all_url(&conn).await?;
    Ok(Json(data))
}

pub async fn shorten(
    Path(url): Path<String>,
    Extension(conn): Extension<PgPool>,
) -> Result<Json<(StatusCode, String)>, AppError> {
    let mut id = nanoid::nanoid!(6);
    let mut exists_in_db = check_if_id_exists(&conn, &id).await?;

    // Keep generating a new ID until it is unique
    while exists_in_db {
        id = nanoid::nanoid!(6);
        exists_in_db = check_if_id_exists(&conn, &id).await?;
    }

    shorten_url(&conn, &url, &id).await?;

    Ok(Json((StatusCode::OK, format!("{}/{}", BASE_URL, id))))

}

pub async fn redirect(
    Path(id): Path<String>,
    Extension(conn): Extension<PgPool>,
) -> Result<Redirect, AppError> {
    let result = get_single_url(&conn, &id).await?;
    Ok(Redirect::to(&result.long_url))
}
