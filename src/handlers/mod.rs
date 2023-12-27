use crate::{
    services::{check_if_id_exists, get_all_url, get_single_url, shorten_url},
    types::{AppError, NewURL, StoredURL, BASE_URL},
};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::Redirect,
    Json,
};
use sqlx::PgPool;


pub async fn hello_world(
    Extension(conn): Extension<PgPool>,
) -> Result<Json<Vec<StoredURL>>, AppError> {
    let data = get_all_url(&conn).await?;
    Ok(Json(data))
}

pub async fn shorten(
    Extension(conn): Extension<PgPool>,
    Path(url): Path<String>,
) -> Result<(StatusCode, Json<NewURL>), AppError> {
    let mut id = nanoid::nanoid!(6);
    let mut exists_in_db = check_if_id_exists(&conn, &id).await?;

    // Keep generating a new ID until it is unique
    while exists_in_db {
        id = nanoid::nanoid!(6);
        exists_in_db = check_if_id_exists(&conn, &id).await?;
    }

    shorten_url(&conn, &url, &id).await?;

    Ok((
        StatusCode::OK,
        Json(NewURL {
            long_url: url,
            short_url: format!("{}/{}", BASE_URL, id),
        }),
    ))
}

pub async fn redirect(
    Path(id): Path<String>,
    Extension(conn): Extension<PgPool>,
) -> Result<Redirect, AppError> {
    let result = get_single_url(&conn, &id).await?;
    Ok(Redirect::to(&result.long_url))
}
