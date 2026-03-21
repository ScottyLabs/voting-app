use dotenvy::{dotenv, from_filename};
use sea_orm::{ConnectionTrait, Database};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load backend/.env first, then voting-app/.env.
    let _ = dotenv();
    if std::env::var("DATABASE_URL").is_err() {
        let _ = from_filename("../.env");
    }

    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL is missing; set it in .env before running this test")?;

    let db = Database::connect(&database_url).await?;

    db.execute_unprepared("SELECT 1;").await?;
    println!("Connected to DATABASE_URL successfully.");
    println!("Executed test query: SELECT 1;");

    Ok(())
}
