use sqlx::PgPool;
use crate::types::{StoredURL, BASE_URL, AppError};

pub async fn check_if_id_exists(conn: &PgPool, id: &str) -> Result<bool, AppError> {
    let result: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM url_mapping WHERE id = $1")
        .bind(id)
        .fetch_one(conn)
        .await?;
    let exists: bool = result > 0;
    Ok(exists)
}
pub async fn shorten_url(conn: &PgPool, url: &str, id: &str) -> Result<(), AppError> {
    sqlx::query("INSERT INTO url_mapping VALUES ($1, $2, $3)")
        .bind(id)
        .bind(url)
        .bind(format!("{}/{}", BASE_URL, id))
        .execute(conn)
        .await?;
    Ok(())
}
pub async fn delete_url(conn: &PgPool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM url_mapping WHERE id = $1")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
}
pub async fn get_single_url(conn: &PgPool, id: &str) -> Result<StoredURL, AppError> {
    let data = sqlx::query_as::<_, StoredURL>("SELECT * FROM url_mapping WHERE id = $1")
        .bind(id)
        .fetch_one(conn)
        .await?;
    Ok(data)
}
pub async fn get_all_url(conn: &PgPool) -> Result<Vec<StoredURL>, AppError> {
    let data = sqlx::query_as::<_, StoredURL>("SELECT * FROM url_mapping")
        .fetch_all(conn)
        .await?;
    Ok(data)
}