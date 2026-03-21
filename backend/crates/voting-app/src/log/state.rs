use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicI32, Ordering},
    },
    time::SystemTime,
};

use sea_orm::{Database, DatabaseConnection, DbErr};
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct StoredRecord {
    pub id: i32,
    pub created_at: SystemTime,
    pub data: Value,
}

#[derive(Clone)]
pub struct AppState {
    pub _db: DatabaseConnection,
    pub records: Arc<Mutex<Vec<StoredRecord>>>,
    pub next_id: Arc<AtomicI32>,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            _db: db,
            records: Arc::new(Mutex::new(Vec::new())),
            next_id: Arc::new(AtomicI32::new(1)),
        }
    }

    pub fn allocate_id(&self) -> i32 {
        self.next_id.fetch_add(1, Ordering::SeqCst)
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
