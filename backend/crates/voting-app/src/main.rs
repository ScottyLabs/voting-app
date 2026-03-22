mod config;
mod core;
mod domain;
mod handlers;
mod server;
mod static_event_reader;

use dotenvy::dotenv;
use sea_orm::DatabaseConnection;
use voting_app_store::Store;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub store: Store,
    pub config: config::Config,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    server::setup().await;
}
