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
                    .table(Meeting::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Meeting::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Meeting::RoomCode).string().not_null())
                    .col(ColumnDef::new(Meeting::Status).string().not_null())
                    .col(
                        ColumnDef::new(Meeting::StartTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Meeting::EndTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Meeting::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Meeting::CreatedByUserId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Meeting::OrganizationId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Meeting::Table, Meeting::CreatedByUserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Meeting::Table, Meeting::OrganizationId)
                            .to(Organization::Table, Organization::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Meeting::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Meeting {
    Table,
    Id,
    RoomCode,
    Status,
    StartTime,
    EndTime,
    CreatedAt,
    CreatedByUserId,
    OrganizationId,
}
