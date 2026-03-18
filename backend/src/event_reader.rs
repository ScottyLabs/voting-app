use entity::event::{self, Entity as Event};
use entity::user::{self, Entity as User};
use entity::vote::{self, Entity as Vote};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use chrono::{DateTime, FixedOffset};
use serde_json::Value as Json;

struct EventLoad {
    event_id: i32,
    event_type: String,
    name: String,
    start_time: DateTime<FixedOffset>,
    end_time: Option<DateTime<FixedOffset>>,
    data: Json,
    created_by_user_id: i32,
    organization_id: i32,
    votes_with_user: Vec<(vote::Model, Option<user::Model>)>,
}

impl EventLoad {
    async fn new(event_id: i32, db: &DatabaseConnection) -> Option<Self> {
        let event = Event::find_by_id(event_id).one(db).await.ok()??;

        let votes = Vote::find()
            .filter(vote::Column::EventId.eq(event_id))
            .all(db)
            .await
            .ok()?;

        let users = votes.load_one(User, db).await.ok()?;

        let votes_with_user = votes.into_iter().zip(users.into_iter()).collect();

        Some(EventLoad {
            event_id: event.id,
            event_type: event.event_type,
            name: event.name,
            start_time: event.start_time.into(),
            end_time: event.end_time.map(|t| t.into()),
            data: event.data,
            created_by_user_id: event.created_by_user_id,
            organization_id: event.organization_id,
            votes_with_user,
        })
    }

    fn vote_count(&self) -> usize {
        self.votes_with_user.len()
    }

    fn get_voters(&self) -> Vec<&user::Model> {
        self.votes_with_user
            .iter()
            .filter_map(|(_, user)| user.as_ref())
            .collect()
    }
}
