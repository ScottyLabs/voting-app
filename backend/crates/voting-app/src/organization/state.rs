use sea_orm::{Database, DatabaseConnection, DbErr};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn build_state() -> Result<AppState, DbErr> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        let database_url_placeholder = "REPLACE_WITH_DATABASE_URL";
        database_url_placeholder.to_owned()
    });

    let db = Database::connect(&database_url).await?;
    Ok(AppState::new(db))
}
