pub mod events;
pub mod organization_members;
pub mod organizations;
pub mod users;
pub mod votes;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
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

    pub fn organization_members(&self) -> organization_members::OrganizationMemberRepository<'_> {
        organization_members::OrganizationMemberRepository::new(&self.db)
    }

    pub fn events(&self) -> events::EventRepository<'_> {
        events::EventRepository::new(&self.db)
    }

    pub fn votes(&self) -> votes::VoteRepository<'_> {
        votes::VoteRepository::new(&self.db)
    }
}
