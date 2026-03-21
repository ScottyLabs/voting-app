mod config;
mod core;
mod domain;
mod server;
mod static_event_reader;

use dotenvy::dotenv;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: config::Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();
    server::setup().await;
}
