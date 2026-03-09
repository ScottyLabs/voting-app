use crate::m20260309_205637_create_meetings::Meeting;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MeetingRole::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(MeetingRole::MeetingId).integer().not_null())
                    .col(ColumnDef::new(MeetingRole::UserId).integer().not_null())
                    .col(ColumnDef::new(MeetingRole::Role).string().not_null())
                    .col(
                        ColumnDef::new(MeetingRole::AssignedByUserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MeetingRole::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .primary_key(
                        Index::create()
                            .col(MeetingRole::MeetingId)
                            .col(MeetingRole::UserId)
                            .col(MeetingRole::Role),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(MeetingRole::Table, MeetingRole::AssignedByUserId)
                            .to(Meeting::Table, Meeting::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MeetingRole::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum MeetingRole {
    Table,
    MeetingId,
    UserId,
    Role,
    AssignedByUserId,
    CreatedAt,
}
