pub use sea_orm_migration::prelude::*;

mod m20260308_183617_create_users;
mod m20260308_191852_create_organizations;
mod m20260308_211556_create_organization_members;
mod m20260310_000844_create_events;
mod m20260313_021648_create_voters;
mod m20260321_223131_create_votes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260308_183617_create_users::Migration),
            Box::new(m20260308_191852_create_organizations::Migration),
            Box::new(m20260308_211556_create_organization_members::Migration),
            Box::new(m20260310_000844_create_events::Migration),
            Box::new(m20260313_021648_create_voters::Migration),
            Box::new(m20260321_223131_create_votes::Migration),
        ]
    }
}
