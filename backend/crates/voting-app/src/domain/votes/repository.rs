#[cfg(feature = "server")]
use crate::entity::{prelude::*, vote};
#[cfg(feature = "server")]
use dioxus::prelude::ServerFnError;
#[cfg(feature = "server")]
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

#[cfg(feature = "server")]
pub struct VoteRepository<'a> {
    db: &'a DatabaseConnection,
}

#[cfg(feature = "server")]
impl<'a> VoteRepository<'a> {
    pub fn new(db: &'a DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn find_proxy_vote_by_user(
        &self,
        user_id: i32,
        event_id: i32,
    ) -> Result<Option<vote::Model>, ServerFnError> {
        Vote::find()
            .filter(vote::Column::VoterId.eq(user_id))
            .filter(vote::Column::EventId.eq(event_id))
            .one(self.db)
            .await
            .map_err(|e| ServerFnError::new(format!("Failed to fetch proxy vote: {}", e)))
    }
}
