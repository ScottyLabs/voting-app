use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm_migration::DbErr;
use std::env;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = sea_orm::Database::connect(&db_url).await?;

    // This applies all pending migrations
    Migrator::up(&connection, None).await?;
    println!("Migration Complete!");

    // Start your server logic...
    Ok(())
}
