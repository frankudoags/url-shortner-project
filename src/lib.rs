use sqlx::postgres::PgPoolOptions;
use std::error::Error;
pub mod routes;

pub async fn run(db_uri: &str) -> Result<(), Box<dyn Error>> {
    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(db_uri)
        .await?;

    sqlx::migrate!().run(&db).await?;

    let app = routes::create_routes(db).await?;

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
