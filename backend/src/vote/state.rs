use sea_orm::{Database, DatabaseConnection, DbErr};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub default_type: String,
    pub open_status: String,
    pub closed_status: String,
}

impl AppState {
    pub fn new(
        db: DatabaseConnection,
        default_type: String,
        open_status: String,
        closed_status: String,
    ) -> Self {
        Self {
            db,
            default_type,
            open_status,
            closed_status,
        }
    }
}

pub async fn build_state() -> Result<AppState, DbErr> {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        let database_url_placeholder = "REPLACE_WITH_DATABASE_URL";
        database_url_placeholder.to_owned()
    });

    let default_type = "MOTION".to_owned();
    let open_status = "OPEN".to_owned();
    let closed_status = "CLOSED".to_owned();

    let db = Database::connect(&database_url).await?;

    Ok(AppState::new(db, default_type, open_status, closed_status))
}
