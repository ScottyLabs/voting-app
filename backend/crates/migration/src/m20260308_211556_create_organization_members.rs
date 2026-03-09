use crate::m20260308_183617_create_users::User;
use crate::m20260308_191852_create_organizations::Organization;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrganizationMember::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganizationMember::OrganizationId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationMember::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganizationMember::MemberState)
                            .string()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(OrganizationMember::OrganizationId)
                            .col(OrganizationMember::UserId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                OrganizationMember::Table,
                                OrganizationMember::OrganizationId,
                            )
                            .to(Organization::Table, Organization::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(OrganizationMember::Table, OrganizationMember::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganizationMember::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum OrganizationMember {
    Table,
    OrganizationId,
    UserId,
    MemberState,
}
