use ::entity::{organization_member, prelude::*};
use sea_orm::*;

pub struct OrganizationMemberRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> OrganizationMemberRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub fn db(&self) -> &DatabaseConnection {
        &self.db
    }

    pub async fn fetch_users_of_organization(
        &self,
        organization_id: i32,
    ) -> Result<Vec<organization_member::Model>, DbErr> {
        OrganizationMember::find()
            .filter(organization_member::Column::OrganizationId.eq(organization_id))
            .all(self.db)
            .await
    }

    pub async fn fetch_organizations_of_user(
        &self,
        user_id: i32,
    ) -> Result<Vec<organization_member::Model>, DbErr> {
        OrganizationMember::find()
            .filter(organization_member::Column::UserId.eq(user_id))
            .all(self.db)
            .await
    }

    pub async fn find_by_organization_and_user(
        &self,
        organization_id: i32,
        user_id: i32,
    ) -> Result<Option<organization_member::Model>, DbErr> {
        OrganizationMember::find_by_id((organization_id, user_id))
            .one(self.db)
            .await
    }

    pub async fn create(
        &self,
        organization_member: organization_member::ActiveModel,
    ) -> Result<organization_member::Model, DbErr> {
        organization_member.insert(self.db).await
    }

    pub async fn update(
        &self,
        organization_member: organization_member::ActiveModel,
    ) -> Result<organization_member::Model, DbErr> {
        organization_member.update(self.db).await
    }

    pub async fn delete(&self, organization_id: i32, user_id: i32) -> Result<DeleteResult, DbErr> {
        OrganizationMember::delete_by_id((organization_id, user_id))
            .exec(self.db)
            .await
    }
}
