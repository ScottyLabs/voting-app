pub mod organizations;
pub mod users;

use sea_orm::DatabaseConnection;

pub struct Store {
    db: DatabaseConnection,
}

impl Store {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub fn users(&self) -> users::UserRepository<'_> {
        users::UserRepository::new(&self.db)
    }

    pub fn organizations(&self) -> organizations::OrganizationRepository<'_> {
        organizations::OrganizationRepository::new(&self.db)
    }
}
