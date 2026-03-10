pub use sea_orm_migration::prelude::*;

mod m20260308_183617_create_users;
mod m20260308_191852_create_organizations;
mod m20260308_211556_create_organization_members;
mod m20260309_205637_create_meetings;
mod m20260309_210931_create_meeting_roles;
mod m20260309_235225_create_attendances;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260308_183617_create_users::Migration),
            Box::new(m20260308_191852_create_organizations::Migration),
            Box::new(m20260308_211556_create_organization_members::Migration),
            Box::new(m20260309_205637_create_meetings::Migration),
            Box::new(m20260309_210931_create_meeting_roles::Migration),
            Box::new(m20260309_235225_create_attendances::Migration),
        ]
    }
}
