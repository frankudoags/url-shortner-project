use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use url_shortner_project::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let db_uri = dotenv!("DATABASE_URL");
    match run(db_uri).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
 