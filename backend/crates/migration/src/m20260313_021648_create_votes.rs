use crate::m20260308_183617_create_users::User;
use crate::m20260310_000844_create_events::Event;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vote::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Vote::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Vote::EventId).integer().not_null())
                    .col(ColumnDef::new(Vote::VoterId).integer().not_null())
                    .col(
                        ColumnDef::new(Vote::CastTime)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Vote::Proxy).boolean().not_null())
                    .col(ColumnDef::new(Vote::Data).json_binary().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Vote::Table, Vote::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Vote::Table, Vote::VoterId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vote::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Vote {
    Table,
    Id,
    EventId,
    VoterId,
    CastTime,
    Proxy,
    Data,
}
