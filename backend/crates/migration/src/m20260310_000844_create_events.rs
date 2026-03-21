use crate::m20260308_183617_create_users::User;
use crate::m20260308_191852_create_organizations::Organization;
use sea_orm_migration::prelude::extension::postgres::Type;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(EventType::Enum)
                    .values([EventType::Motion, EventType::Election])
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(StatusOption::Enum)
                    .values([StatusOption::Active, StatusOption::Inactive])
                    .to_owned(),
            )
            .await?;

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
                    .col(
                        ColumnDef::new(Event::EventType)
                            .custom(EventType::Enum)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Event::Name).string().not_null())
                    .col(
                        ColumnDef::new(Event::Status)
                            .custom(StatusOption::Enum)
                            .not_null(),
                    )
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
            .await?;
        manager
            .drop_type(Type::drop().name(EventType::Enum).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(StatusOption::Enum).to_owned())
            .await?;

        Ok(())
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

#[derive(DeriveIden)]
pub enum EventType {
    #[sea_orm(iden = "event_type")]
    Enum,
    Motion,
    Election,
}

#[derive(DeriveIden)]
pub enum StatusOption {
    #[sea_orm(iden = "status_option")]
    Enum,
    Active,
    Inactive,
}
