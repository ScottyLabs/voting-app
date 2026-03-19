use ::entity::{prelude::*, vote};
use sea_orm::*;

pub struct VoteRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> VoteRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<vote::Model>, DbErr> {
        Vote::find_by_id(id).one(self.db).await
    }

    pub async fn find_by_event_id(&self, id: i32) -> Result<Vec<vote::Model>, DbErr> {
        Vote::find()
            .filter(vote::Column::EventId.eq(id))
            .all(self.db)
            .await
    }

    pub async fn create(&self, vote: vote::ActiveModel) -> Result<vote::Model, DbErr> {
        vote.insert(self.db).await
    }

    pub async fn update(&self, vote: vote::ActiveModel) -> Result<vote::Model, DbErr> {
        vote.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        Vote::delete_by_id(id).exec(self.db).await
    }
}
