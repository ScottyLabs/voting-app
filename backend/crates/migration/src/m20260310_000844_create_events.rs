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
                    .table(Event::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Event::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Event::EventType).string().not_null())
                    .col(ColumnDef::new(Event::Name).string().not_null())
                    .col(ColumnDef::new(Event::Status).string().not_null())
                    .col(
                        ColumnDef::new(Event::StartTime)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Event::EndTime).timestamp_with_time_zone())
                    .col(ColumnDef::new(Event::Data).json_binary().not_null())
                    .col(ColumnDef::new(Event::CreatedByUserId).integer().not_null())
                    .col(ColumnDef::new(Event::OrganizationId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Event::Table, Event::CreatedByUserId)
                            .to(User::Table, User::Id)
                            // this may not be the correct methodology
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Event::Table, Event::OrganizationId)
                            .to(Organization::Table, Organization::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Event::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Event {
    Table,
    Id,
    EventType,
    Name,
    Status,
    StartTime,
    EndTime,
    Data,
    CreatedByUserId,
    OrganizationId,
}
