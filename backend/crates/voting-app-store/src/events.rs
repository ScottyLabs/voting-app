use ::entity::{event, prelude::*};
use sea_orm::*;

pub struct EventRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> EventRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<event::Model>, DbErr> {
        Event::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, event: event::ActiveModel) -> Result<event::Model, DbErr> {
        event.insert(self.db).await
    }

    pub async fn update(&self, event: event::ActiveModel) -> Result<event::Model, DbErr> {
        event.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        Event::delete_by_id(id).exec(self.db).await
    }

    pub async fn find_active_by_session_code(
        &self,
        session_code: &str,
    ) -> Result<Option<event::Model>, DbErr> {
        Event::find()
            .filter(event::Column::Status.eq("active"))
            .all(self.db)
            .await
            .map(|events| {
                events.into_iter().find(|e| {
                    e.data
                        .get("session_code")
                        .and_then(|v| v.as_str())
                        .map(|code| code == session_code)
                        .unwrap_or(false)
                })
            })
    }
}
