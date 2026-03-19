use ::entity::{prelude::*, user};
use sea_orm::*;

pub struct UserRepository<'a> {
    db: &'a DatabaseConnection,
}

impl<'a> UserRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<user::Model>, DbErr> {
        User::find_by_id(id).one(self.db).await
    }

    pub async fn create(&self, user: user::ActiveModel) -> Result<user::Model, DbErr> {
        user.insert(self.db).await
    }

    pub async fn update(&self, user: user::ActiveModel) -> Result<user::Model, DbErr> {
        user.update(self.db).await
    }

    pub async fn delete(&self, id: i32) -> Result<DeleteResult, DbErr> {
        User::delete_by_id(id).exec(self.db).await
    }
}
