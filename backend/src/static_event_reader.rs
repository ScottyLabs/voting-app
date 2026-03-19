use entity::event::{self, Entity as Event};
use entity::user::{self, Entity as User};
use entity::vote::{self, Entity as Vote};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use std::collections::HashMap;

//Struct for exporting vote.
struct VoteResult {
    user_id: i32,
    user_name: String,
    vote_timestamp: DateTime<FixedOffset>,
    vote_response: Vec<String>, //parsed from vote.data (JSON)
}

// parsed from event.data JSON blob
#[derive(Deserialize)]
struct Visibility {
    participants: String, // "hidden_until_release" | "live"
}

#[derive(Deserialize)]
struct EventData {
    description: String,
    session_code: String,
    vote_type: String,          // "motion" | "election"
    threshold: f64,             // approval threshold e.g. 0.75 = 75%
    visibility: Visibility,
    proxy: bool,
    vote_options: Vec<String>,
}

struct EventLoadStatic {
    event_id: i32,
    event_type: String,
    name: String,
    status: String,
    start_time: DateTime<FixedOffset>,          //timestamp with timezone
    end_time: Option<DateTime<FixedOffset>>,    //nullable timestamp
    data: EventData,                            //parsed event.data JSON
    created_by_user_id: i32,
    organization_id: i32,
    votes_with_user: Vec<(vote::Model, Option<user::Model>)>,
    // list of tuples. Each vote paired with its user.
    //For now, user might be Null
}

impl EventLoadStatic {
    //EventLoadStatic constructor. return SOME(EventLoadStatic) for sc, NONE for fc
    async fn new(event_id: i32, db: &DatabaseConnection) -> Option<Self> {
        let event = Event::find_by_id(event_id).one(db).await.ok()??;

        let votes = Vote::find()
            .filter(vote::Column::EventId.eq(event_id))
            .all(db)
            .await
            .ok()?;

        let users = votes.load_one(User, db).await.ok()?;
        let votes_with_user = votes.into_iter().zip(users.into_iter()).collect();
        //parse event.data (JSON)
        let data: EventData = serde_json::from_value(event.data).ok()?;
        Some(EventLoadStatic {
            event_id: event.id,
            event_type: event.event_type,
            name: event.name,
            status: event.status,
            start_time: event.start_time.into(),
            end_time: event.end_time.map(|t| t.into()),
            data,
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

    //equiv to get_attendance. 
    //return vector containing VoteResult struct
    fn get_event_result(&self) -> Vec<VoteResult> {
        self.votes_with_user
            .iter()
            .filter_map(|(vote, user)| { //vote : vote::Model, user : Option<user::Model>
                let user = user.as_ref()?;
                let vote_response = vote.data
                    .get("vote_response")?
                    .as_array()?
                    .iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                    .collect();

                Some(VoteResult {
                    user_id: user.id,
                    user_name: user.name.clone(),
                    vote_timestamp: vote.cast_time,
                    vote_response,
                })
            })
            .collect()
    }
    
    // returns HashMap of response -> count
    // e.g. {"yes": 10, "no": 5}
    fn get_vote_statistics(&self) -> HashMap<String, usize> {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for result in self.get_event_result() {
            for response in result.vote_response {
                *counts.entry(response).or_insert(0) += 1;
            }
        }
        counts
    }

    //return val: raw PDF bytes. caller should handle this 
    //(idealy just shot it to frontend and let it streamed)
    fn export_result_pdf(&self) -> Vec<u8> {
        let mut doc = genpdf::Document::new(
            genpdf::fonts::from_files("./fonts", "LiberationSans", None)
                .expect("Failed to load fonts"),
        );
        doc.set_title(format!("Event Result: {}", self.name));
        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(10);
        doc.set_page_decorator(decorator);
        // title
        doc.push(genpdf::elements::Paragraph::new(format!(
            "Event: {}",
            self.name
        )));
        doc.push(genpdf::elements::Paragraph::new(format!(
            "Vote Type: {}",
            self.data.vote_type
        )));
        doc.push(genpdf::elements::Paragraph::new(format!(
            "Total Votes: {}",
            self.vote_count()
        )));
        doc.push(genpdf::elements::Break::new(1));

        // statistics
        doc.push(genpdf::elements::Paragraph::new("--- Results ---"));
        for (response, count) in self.get_vote_statistics() {
            doc.push(genpdf::elements::Paragraph::new(format!(
                "{}: {} votes",
                response, count
            )));
        }
        doc.push(genpdf::elements::Break::new(1));

        // table header
        let mut table = genpdf::elements::TableLayout::new(vec![1, 2, 2, 3]);
        table.set_cell_decorator(genpdf::frames::FrameCellDecorator::new(true, true, false));
        let mut row = table.row();
        row.push_element(genpdf::elements::Paragraph::new("User ID"));
        row.push_element(genpdf::elements::Paragraph::new("Name"));
        row.push_element(genpdf::elements::Paragraph::new("Timestamp"));
        row.push_element(genpdf::elements::Paragraph::new("Response"));
        row.push().expect("Failed to push header row");

        // table rows
        for result in self.get_event_result() {
            let mut row = table.row();
            row.push_element(genpdf::elements::Paragraph::new(result.user_id.to_string()));
            row.push_element(genpdf::elements::Paragraph::new(result.user_name));
            row.push_element(genpdf::elements::Paragraph::new(result.vote_timestamp.to_string()));
            row.push_element(genpdf::elements::Paragraph::new(result.vote_response.join(", ")));
            row.push().expect("Failed to push row");
        }
        doc.push(table);

        //byte rendering
        let mut buf = Vec::new();
        doc.render(&mut buf).expect("Failed to render PDF");
        buf
    }
}
