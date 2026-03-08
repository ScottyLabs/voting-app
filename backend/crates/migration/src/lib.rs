pub use sea_orm_migration::prelude::*;

mod m20260308_183617_create_users;
mod m20260308_191852_create_organizations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260308_183617_create_users::Migration),
            Box::new(m20260308_191852_create_organizations::Migration),
        ]
    }
}
