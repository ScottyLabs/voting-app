use ::entity::{organization, prelude::*};
use sea_orm::*;

pub struct OrganizationRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> OrganizationRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<organization::Model>, DbErr> {
        Organization::find_by_id(id).one(self.db).await
    }

    pub async fn create(
        &self,
        organization: organization::ActiveModel,
    ) -> Result<organization::Model, DbErr> {
        organization.insert(self.db).await
    }

    pub async fn update(
        &self,
        organization: organization::ActiveModel,
    ) -> Result<organization::Model, DbErr> {
        organization.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        Organization::delete_by_id(id).exec(self.db).await
    }
}
