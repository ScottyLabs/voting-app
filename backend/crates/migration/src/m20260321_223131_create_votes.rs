use crate::m20260321_223131_create_voter::Voter;
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
                    .col(ColumnDef::new(Vote::Id).integer().not_null().primary_key())
                    .col(
                        ColumnDef::new(Vote::CastTime)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(Vote::Data).json_binary().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Vote::Table, Vote::Id)
                            .to(Voter::Table, Voter::Id)
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
    CastTime,
    Data,
}
