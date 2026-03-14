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
                    .table(Voter::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Voter::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Voter::EventId).integer().not_null())
                    .col(ColumnDef::new(Voter::UserId).integer())
                    .col(ColumnDef::new(Voter::Data).json_binary().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Voter::Table, Voter::EventId)
                            .to(Event::Table, Event::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Voter::Table, Voter::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Voter::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Voter {
    Table,
    Id,
    EventId,
    UserId,
    Data,
}
