// Loads event + votes into memory once. Useful for repeated reads (export, statistics)
// without re-querying the DB. Consider caching for closed events.
use chrono::{DateTime, FixedOffset};
use entity::event::Entity as Event;
use entity::user::{self, Entity as User};
use entity::vote::{self, Entity as Vote};
use genpdf::elements::FrameCellDecorator;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
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
    vote_type: String, // "motion" | "election"
    threshold: f64,    // approval threshold e.g. 0.75 = 75%
    visibility: Visibility,
    proxy: bool,
    vote_options: Vec<String>,
}

//EventTable struct
struct EventLoadStatic {
    event_id: i32,
    event_type: String,
    name: String,
    status: String,
    start_time: DateTime<FixedOffset>, //timestamp with timezone
    end_time: Option<DateTime<FixedOffset>>, //nullable timestamp
    data: EventData,                   //parsed event.data JSON
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
            .filter_map(|(vote, user)| {
                //vote : vote::Model, user : Option<user::Model>
                let user = user.as_ref()?;
                let vote_response = vote
                    .data
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
    //(idealy just shot it to frontend and let it streamed
    fn export_result_pdf(&self) -> Vec<u8> {
        let font_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fonts");
        let font_family = genpdf::fonts::FontFamily {
            regular: genpdf::fonts::FontData::new(
                std::fs::read(format!("{}/liberation-sans.regular.ttf", font_dir))
                    .expect("Failed to read regular font"),
                None,
            )
            .expect("Failed to load regular font"),
            bold: genpdf::fonts::FontData::new(
                std::fs::read(format!("{}/liberation-sans.bold.ttf", font_dir))
                    .expect("Failed to read bold font"),
                None,
            )
            .expect("Failed to load bold font"),
            italic: genpdf::fonts::FontData::new(
                std::fs::read(format!("{}/liberation-sans.italic.ttf", font_dir))
                    .expect("Failed to read italic font"),
                None,
            )
            .expect("Failed to load italic font"),
            bold_italic: genpdf::fonts::FontData::new(
                std::fs::read(format!("{}/liberation-sans.bold-italic.ttf", font_dir))
                    .expect("Failed to read bold-italic font"),
                None,
            )
            .expect("Failed to load bold-italic font"),
        };
        let mut doc = genpdf::Document::new(font_family);
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
        table.set_cell_decorator(FrameCellDecorator::new(true, true, false));
        // helper: center text horizontally and add vertical padding
        let cell = |text: &str| {
            genpdf::elements::PaddedElement::new(
                genpdf::elements::Paragraph::new(text).aligned(genpdf::Alignment::Center),
                genpdf::Margins::trbl(2, 1, 2, 1),
            )
        };

        let mut row = table.row();
        row.push_element(cell("User ID"));
        row.push_element(cell("Name"));
        row.push_element(cell("Timestamp"));
        row.push_element(cell("Response"));
        row.push().expect("Failed to push header row");

        // table rows
        for result in self.get_event_result() {
            let timestamp = result
                .vote_timestamp
                .format("%Y-%m-%d %H:%M:%S")
                .to_string();
            let mut row = table.row();
            row.push_element(cell(&result.user_id.to_string()));
            row.push_element(cell(&result.user_name));
            row.push_element(cell(&timestamp));
            row.push_element(cell(&result.vote_response.join(", ")));
            row.push().expect("Failed to push row");
        }
        doc.push(table);

        //byte rendering
        let mut buf = Vec::new();
        doc.render(&mut buf).expect("Failed to render PDF");
        buf
    }

    //for frontent
    fn export_result_json(&self) -> serde_json::Value {
        let statistics: serde_json::Map<String, serde_json::Value> = self
            .get_vote_statistics()
            .into_iter()
            .map(|(k, v)| (k, serde_json::Value::from(v)))
            .collect();

        serde_json::json!({
            "event_name": self.name,
            "vote_type": self.data.vote_type,
            "status": self.status,
            "total_votes": self.vote_count(),
            "end_time": self.end_time.map(|t| t.to_rfc3339()),
            "statistics": statistics
        })
    }
}

//test cases are 100% AI generated. Cannot garuntee safety.
//run cargo test -- --ignored --nocapture && open /tmp/test_event_result.pdf to preview pdf formatting.
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;
    fn mock_user(id: i32, name: &str) -> user::Model {
        user::Model {
            id,
            name: name.to_string(),
            created_at: Utc::now().fixed_offset(),
        }
    }

    fn mock_vote(id: i32, event_id: i32, voter_id: i32, responses: Vec<&str>) -> vote::Model {
        vote::Model {
            id,
            event_id,
            voter_id,
            cast_time: Utc::now().fixed_offset(),
            proxy: false,
            data: json!({
                "vote_type": "motion",
                "vote_response": responses
            }),
        }
    }

    fn mock_event(votes_with_user: Vec<(vote::Model, Option<user::Model>)>) -> EventLoadStatic {
        EventLoadStatic {
            event_id: 1,
            event_type: "vote".to_string(),
            name: "Test Event".to_string(),
            status: "closed".to_string(),
            start_time: Utc::now().fixed_offset(),
            end_time: None,
            data: EventData {
                description: "test description".to_string(),
                session_code: "ABC123".to_string(),
                vote_type: "motion".to_string(),
                threshold: 0.75,
                visibility: Visibility {
                    participants: "live".to_string(),
                },
                proxy: false,
                vote_options: vec!["yes".to_string(), "no".to_string()],
            },
            created_by_user_id: 1,
            organization_id: 1,
            votes_with_user,
        }
    }

    #[test]
    fn test_vote_count_empty() {
        let event = mock_event(vec![]);
        assert_eq!(event.vote_count(), 0);
    }

    #[test]
    fn test_vote_count() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["no"]), Some(mock_user(2, "Bob"))),
        ];
        let event = mock_event(votes);
        assert_eq!(event.vote_count(), 2);
    }

    #[test]
    fn test_get_voters() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["no"]), None), // no user
        ];
        let event = mock_event(votes);
        let voters = event.get_voters();
        assert_eq!(voters.len(), 1); // only Alice, None is skipped
        assert_eq!(voters[0].name, "Alice");
    }

    #[test]
    fn test_get_event_result() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["no"]), Some(mock_user(2, "Bob"))),
        ];
        let event = mock_event(votes);
        let results = event.get_event_result();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].user_name, "Alice");
        assert_eq!(results[0].vote_response, vec!["yes"]);
        assert_eq!(results[1].user_name, "Bob");
        assert_eq!(results[1].vote_response, vec!["no"]);
    }

    #[test]
    fn test_get_event_result_skips_missing_user() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["no"]), None), // skipped
        ];
        let event = mock_event(votes);
        let results = event.get_event_result();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_get_vote_statistics() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["yes"]), Some(mock_user(2, "Bob"))),
            (mock_vote(3, 1, 3, vec!["no"]), Some(mock_user(3, "Carol"))),
        ];
        let event = mock_event(votes);
        let stats = event.get_vote_statistics();

        assert_eq!(stats.get("yes"), Some(&2));
        assert_eq!(stats.get("no"), Some(&1));
    }

    #[test]
    fn test_get_vote_statistics_empty() {
        let event = mock_event(vec![]);
        let stats = event.get_vote_statistics();
        assert!(stats.is_empty());
    }

    #[test]
    fn test_export_result_json() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["yes"]), Some(mock_user(2, "Bob"))),
            (mock_vote(3, 1, 3, vec!["no"]), Some(mock_user(3, "Carol"))),
        ];
        let event = mock_event(votes);
        let result = event.export_result_json();

        assert_eq!(result["event_name"], "Test Event");
        assert_eq!(result["vote_type"], "motion");
        assert_eq!(result["status"], "closed");
        assert_eq!(result["total_votes"], 3);
        assert!(result["end_time"].is_null());
        assert_eq!(result["statistics"]["yes"], 2);
        assert_eq!(result["statistics"]["no"], 1);
    }

    #[test]
    fn test_export_result_json_empty() {
        let event = mock_event(vec![]);
        let result = event.export_result_json();

        assert_eq!(result["event_name"], "Test Event");
        assert_eq!(result["total_votes"], 0);
        assert!(result["statistics"].as_object().unwrap().is_empty());
    }

    #[test]
    fn test_export_result_json_with_end_time() {
        let mut event = mock_event(vec![(
            mock_vote(1, 1, 1, vec!["yes"]),
            Some(mock_user(1, "Alice")),
        )]);
        event.end_time = Some(chrono::Utc::now().fixed_offset());
        let result = event.export_result_json();

        assert!(!result["end_time"].is_null());
        // end_time should be an RFC3339 string
        assert!(result["end_time"].as_str().is_some());
    }

    #[test]
    #[ignore = "requires LiberationSans font files in ./fonts directory"]
    fn test_export_result_pdf_returns_bytes() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["no"]), Some(mock_user(2, "Bob"))),
        ];
        let event = mock_event(votes);
        let bytes = event.export_result_pdf();
        assert!(!bytes.is_empty());
        // PDF files always start with "%PDF"
        assert_eq!(&bytes[..4], b"%PDF");
    }

    #[test]
    #[ignore = "requires LiberationSans font files in ./fonts directory"]
    fn test_export_result_pdf_empty_votes() {
        let event = mock_event(vec![]);
        let bytes = event.export_result_pdf();
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");
    }

    #[test]
    #[ignore = "requires LiberationSans font files in ./fonts directory"]
    fn test_export_result_pdf_saves_to_file() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["no"]), Some(mock_user(2, "Bob"))),
        ];
        let event = mock_event(votes);
        let bytes = event.export_result_pdf();

        let path = "/tmp/test_event_result.pdf";
        std::fs::write(path, &bytes).expect("Failed to write PDF");
        assert!(std::path::Path::new(path).exists());

        // cleanup
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    #[ignore = "requires LiberationSans font files in ./fonts directory"]
    fn test_export_result_pdf_speed() {
        // small event — 2 votes
        let votes_small = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["no"]), Some(mock_user(2, "Bob"))),
        ];

        // large event — 100 votes
        let votes_large: Vec<_> = (1..=100)
            .map(|i| {
                (
                    mock_vote(i, 1, i, vec!["yes"]),
                    Some(mock_user(i, &format!("User{}", i))),
                )
            })
            .collect();

        let event_small = mock_event(votes_small);
        let event_large = mock_event(votes_large);

        // measure small event
        let start = std::time::Instant::now();
        let _ = event_small.export_result_pdf();
        let small_total = start.elapsed();

        // measure large event
        let start = std::time::Instant::now();
        let _ = event_large.export_result_pdf();
        let large_total = start.elapsed();

        // font loading is the first thing export_result_pdf does — measure it separately
        let font_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/fonts");
        let start = std::time::Instant::now();
        let _ = genpdf::fonts::FontFamily {
            regular: genpdf::fonts::FontData::new(
                std::fs::read(format!("{}/liberation-sans.regular.ttf", font_dir)).unwrap(),
                None,
            )
            .unwrap(),
            bold: genpdf::fonts::FontData::new(
                std::fs::read(format!("{}/liberation-sans.bold.ttf", font_dir)).unwrap(),
                None,
            )
            .unwrap(),
            italic: genpdf::fonts::FontData::new(
                std::fs::read(format!("{}/liberation-sans.italic.ttf", font_dir)).unwrap(),
                None,
            )
            .unwrap(),
            bold_italic: genpdf::fonts::FontData::new(
                std::fs::read(format!("{}/liberation-sans.bold-italic.ttf", font_dir)).unwrap(),
                None,
            )
            .unwrap(),
        };
        let font_duration = start.elapsed();

        println!("Font loading:                 {:?}", font_duration);
        println!("PDF export (2 votes):         {:?}", small_total);
        println!("PDF export (100 votes):       {:?}", large_total);
        println!(
            "PDF gen only (2 votes):       {:?}",
            small_total.saturating_sub(font_duration)
        );
        println!(
            "PDF gen only (100 votes):     {:?}",
            large_total.saturating_sub(font_duration)
        );

        assert!(
            large_total.as_secs() < 5,
            "PDF export took too long: {:?}",
            large_total
        );
    }
}
