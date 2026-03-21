mod config;
mod core;
mod domain;
mod handlers;
mod server;
mod static_event_reader;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use std::env;
use voting_app_store::Store;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: config::Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = sea_orm::Database::connect(&db_url).await?;

    Migrator::up(&db, None).await;
    println!("Migration complete!");

    let store = Store::new(db);

    let app = Router::new()
        .route(
            "/api/:session_code/attendance",
            get(handlers::attendance::join),
        )
        .with_state(store);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    println!("Listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.expect("Server error");

    tracing_subscriber::fmt::init();
    server::setup().await;
}
