use crate::m20260308_183617_create_users::User;
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
                    .table(Attendance::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Attendance::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Attendance::EventType).string().not_null()) // what is this
                    // for??
                    .col(
                        ColumnDef::new(Attendance::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Attendance::MeetingId).integer().not_null())
                    .col(ColumnDef::new(Attendance::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Attendance::Table, Attendance::MeetingId)
                            .to(Meeting::Table, Meeting::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Attendance::Table, Attendance::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Attendance::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Attendance {
    Table,
    Id,
    EventType,
    CreatedAt,
    MeetingId,
    UserId,
}
