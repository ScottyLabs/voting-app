// Loads event + votes into memory once. Useful for repeated reads (export, statistics)
// without re-querying the DB. Consider caching for closed events. 
use chrono::{DateTime, FixedOffset};
use entity::event::Entity as Event;
use entity::user::{self, Entity as User};
use entity::vote::{self, Entity as Vote};
use genpdf::elements::FrameCellDecorator;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use entity::event::EventType;

// #[derive(Deserialize, Serialize, PartialEq)]
// #[serde(rename_all = "lowercase")]
// enum EventType {
//     Motion,
//     Election,
// }

#[derive(Deserialize, Serialize)]
enum ParticipantVisibility {
    #[serde(rename = "hidden_until_release")]
    HiddenUntilRelease,
    #[serde(rename = "live")]
    Live,
}

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
    participants: ParticipantVisibility,
}

#[derive(Deserialize)]
struct EventData {
    description: String,
    session_code: String,
    threshold: f64,    // approval threshold e.g. 0.75 = 75%
    visibility: Visibility,
    proxy: bool,
    vote_options: Vec<String>,
}

//EventTable struct
struct EventLoadStatic {
    event_id: i32,
    event_type: EventType,
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
            // event_type: serde_json::from_value(serde_json::Value::String(event.event_type)).ok()?,
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

    // returns Vec where index = rank position, value = HashMap of option -> count
    // e.g. index 0 (1st choice): {"Alice": 5, "Bob": 3}
    //      index 1 (2nd choice): {"Bob": 4, "Carol": 4}
    fn get_ranked_statistics(&self) -> Vec<HashMap<String, usize>> {
        let mut rank_counts: Vec<HashMap<String, usize>> = Vec::new();
        for result in self.get_event_result() {
            for (rank, option) in result.vote_response.iter().enumerate() {
                if rank_counts.len() <= rank {
                    rank_counts.resize(rank + 1, HashMap::new());
                }
                *rank_counts[rank].entry(option.clone()).or_insert(0) += 1;
            }
        }
        rank_counts
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
            "Event Type: {}",
            serde_json::to_value(&self.event_type)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_owned()))
                .unwrap_or_default()
        )));
        doc.push(genpdf::elements::Paragraph::new(format!(
            "Total Votes: {}",
            self.vote_count()
        )));
        doc.push(genpdf::elements::Break::new(1));

        // statistics — branched by event type
        doc.push(genpdf::elements::Paragraph::new("--- Results ---"));
        let total = self.vote_count();
        if self.event_type == EventType::Motion {
            for (response, count) in self.get_vote_statistics() {
                let pct = if total > 0 { count * 100 / total } else { 0 };
                doc.push(genpdf::elements::Paragraph::new(format!(
                    "{}: {} ({}%)",
                    response, count, pct
                )));
            }
        } else {
            // ranked choice: per-rank breakdown
            let ordinal = |n: usize| {
                let suffix = match n % 100 {
                    11 | 12 | 13 => "th",
                    _ => match n % 10 {
                        1 => "st",
                        2 => "nd",
                        3 => "rd",
                        _ => "th",
                    },
                };
                format!("{}{}", n, suffix)
            };
            for (rank, counts) in self.get_ranked_statistics().iter().enumerate() {
                let rank_total: usize = counts.values().sum();
                doc.push(genpdf::elements::Paragraph::new(format!(
                    "{}:",
                    ordinal(rank + 1)
                )));
                for (option, count) in counts {
                    let pct = if rank_total > 0 { count * 100 / rank_total } else { 0 };
                    doc.push(genpdf::elements::Paragraph::new(format!(
                        "  {}: {} ({}%)",
                        option, count, pct
                    )));
                }
            }
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
        let total = self.vote_count();
        let statistics = if self.event_type == EventType::Motion {
            // flat: { "yes": { "count": 2, "pct": 67 }, ... }
            let map: serde_json::Map<String, serde_json::Value> = self
                .get_vote_statistics()
                .into_iter()
                .map(|(k, count)| {
                    let pct = if total > 0 { count * 100 / total } else { 0 };
                    (k, serde_json::json!({ "count": count, "pct": pct }))
                })
                .collect();
            serde_json::Value::Object(map)
        } else {
            // ranked: [ { "rank": 1, "results": { "Alice": { "count": 2, "pct": 67 }, ... } }, ... ]
            let ranked: Vec<serde_json::Value> = self
                .get_ranked_statistics()
                .into_iter()
                .enumerate()
                .map(|(rank, counts)| {
                    let rank_total: usize = counts.values().sum();
                    let results: serde_json::Map<String, serde_json::Value> = counts
                        .into_iter()
                        .map(|(option, count)| {
                            let pct = if rank_total > 0 { count * 100 / rank_total } else { 0 };
                            (option, serde_json::json!({ "count": count, "pct": pct }))
                        })
                        .collect();
                    serde_json::json!({ "rank": rank + 1, "results": results })
                })
                .collect();
            serde_json::Value::Array(ranked)
        };

        serde_json::json!({
            "event_name": self.name,
            "event_type": self.event_type,
            "status": self.status,
            "total_votes": total,
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
                "vote_response": responses
            }),
        }
    }

    fn mock_event(votes_with_user: Vec<(vote::Model, Option<user::Model>)>) -> EventLoadStatic {
        EventLoadStatic {
            event_id: 1,
            event_type: EventType::Motion,
            name: "Test Event".to_string(),
            status: "closed".to_string(),
            start_time: Utc::now().fixed_offset(),
            end_time: None,
            data: EventData {
                description: "test description".to_string(),
                session_code: "ABC123".to_string(),
                threshold: 0.75,
                visibility: Visibility {
                    participants: ParticipantVisibility::Live,
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
    fn test_get_ranked_statistics() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["Alice", "Bob", "Carol"]), Some(mock_user(1, "Voter1"))),
            (mock_vote(2, 1, 2, vec!["Bob", "Alice", "Carol"]), Some(mock_user(2, "Voter2"))),
            (mock_vote(3, 1, 3, vec!["Alice", "Carol", "Bob"]), Some(mock_user(3, "Voter3"))),
        ];
        let mut event = mock_event(votes);
        event.event_type = EventType::Election;
        let ranked = event.get_ranked_statistics();

        // 1st choice: Alice x2, Bob x1
        assert_eq!(ranked[0].get("Alice"), Some(&2));
        assert_eq!(ranked[0].get("Bob"), Some(&1));
        // 2nd choice: Bob x1, Alice x1, Carol x1
        assert_eq!(ranked[1].get("Bob"), Some(&1));
        assert_eq!(ranked[1].get("Alice"), Some(&1));
        assert_eq!(ranked[1].get("Carol"), Some(&1));
        // 3rd choice: Carol x2, Bob x1
        assert_eq!(ranked[2].get("Carol"), Some(&2));
        assert_eq!(ranked[2].get("Bob"), Some(&1));
    }

    #[test]
    fn test_get_ranked_statistics_empty() {
        let event = mock_event(vec![]);
        let ranked = event.get_ranked_statistics();
        assert!(ranked.is_empty());
    }

    #[test]
    fn test_export_result_json_election() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["Alice", "Bob"]), Some(mock_user(1, "Voter1"))),
            (mock_vote(2, 1, 2, vec!["Alice", "Bob"]), Some(mock_user(2, "Voter2"))),
            (mock_vote(3, 1, 3, vec!["Bob", "Alice"]), Some(mock_user(3, "Voter3"))),
        ];
        let mut event = mock_event(votes);
        event.event_type = EventType::Election;
        let result = event.export_result_json();

        let stats = result["statistics"].as_array().unwrap();
        // rank 1
        assert_eq!(stats[0]["rank"], 1);
        assert_eq!(stats[0]["results"]["Alice"]["count"], 2);
        assert_eq!(stats[0]["results"]["Bob"]["count"], 1);
        // rank 2
        assert_eq!(stats[1]["rank"], 2);
        assert_eq!(stats[1]["results"]["Bob"]["count"], 2);
        assert_eq!(stats[1]["results"]["Alice"]["count"], 1);
    }

    #[test]
    fn test_export_result_json_election_pct() {
        // 4 voters: Alice gets 3 first-choice votes (75%), Bob gets 1 (25%)
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["Alice", "Bob"]), Some(mock_user(1, "Voter1"))),
            (mock_vote(2, 1, 2, vec!["Alice", "Bob"]), Some(mock_user(2, "Voter2"))),
            (mock_vote(3, 1, 3, vec!["Alice", "Bob"]), Some(mock_user(3, "Voter3"))),
            (mock_vote(4, 1, 4, vec!["Bob", "Alice"]), Some(mock_user(4, "Voter4"))),
        ];
        let mut event = mock_event(votes);
        event.event_type = EventType::Election;
        let result = event.export_result_json();

        let stats = result["statistics"].as_array().unwrap();
        assert_eq!(stats[0]["results"]["Alice"]["pct"], 75);
        assert_eq!(stats[0]["results"]["Bob"]["pct"], 25);
        // rank 2: Bob x3 (75%), Alice x1 (25%)
        assert_eq!(stats[1]["results"]["Bob"]["pct"], 75);
        assert_eq!(stats[1]["results"]["Alice"]["pct"], 25);
    }

    #[test]
    fn test_get_ranked_statistics_unequal_lengths() {
        // voter1 ranks 3, voter2 ranks only 1
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["Alice", "Bob", "Carol"]), Some(mock_user(1, "Voter1"))),
            (mock_vote(2, 1, 2, vec!["Bob"]), Some(mock_user(2, "Voter2"))),
        ];
        let event = mock_event(votes);
        let ranked = event.get_ranked_statistics();

        assert_eq!(ranked.len(), 3); // max rank depth is 3
        assert_eq!(ranked[0].get("Alice"), Some(&1));
        assert_eq!(ranked[0].get("Bob"), Some(&1));
        assert_eq!(ranked[1].get("Bob"), Some(&1)); // only voter1 ranked 2nd
        assert_eq!(ranked[2].get("Carol"), Some(&1)); // only voter1 ranked 3rd
    }

    #[test]
    #[ignore = "requires LiberationSans font files in ./fonts directory"]
    fn test_export_result_pdf_election_returns_bytes() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["Alice", "Bob", "Carol"]), Some(mock_user(1, "Voter1"))),
            (mock_vote(2, 1, 2, vec!["Bob", "Alice", "Carol"]), Some(mock_user(2, "Voter2"))),
        ];
        let mut event = mock_event(votes);
        event.event_type = EventType::Election;
        let bytes = event.export_result_pdf();
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[..4], b"%PDF");
    }

    #[test]
    fn test_export_result_json_preview() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["no"]), Some(mock_user(2, "Bob"))),
            (mock_vote(3, 1, 3, vec!["yes"]), Some(mock_user(3, "Carol"))),
        ];
        let event = mock_event(votes);
        let result = event.export_result_json();
        println!("{}", serde_json::to_string_pretty(&result).unwrap());
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
        assert_eq!(result["status"], "closed");
        assert_eq!(result["total_votes"], 3);
        assert!(result["end_time"].is_null());
        assert_eq!(result["statistics"]["yes"]["count"], 2);
        assert_eq!(result["statistics"]["yes"]["pct"], 66);
        assert_eq!(result["statistics"]["no"]["count"], 1);
        assert_eq!(result["statistics"]["no"]["pct"], 33);
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
    #[ignore = "preview only — writes pdf to /tmp and does not clean up"]
    fn test_export_result_pdf_preview() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["yes"]), Some(mock_user(1, "Alice"))),
            (mock_vote(2, 1, 2, vec!["no"]), Some(mock_user(2, "Bob"))),
            (mock_vote(3, 1, 3, vec!["yes"]), Some(mock_user(3, "Carol"))),
        ];
        let event = mock_event(votes);
        let bytes = event.export_result_pdf();
        std::fs::write("/tmp/test_event_result.pdf", &bytes).expect("Failed to write PDF");
    }

    #[test]
    #[ignore = "preview only — writes pdf to /tmp and does not clean up"]
    fn test_export_result_pdf_preview_election() {
        let votes = vec![
            (mock_vote(1, 1, 1, vec!["Alice", "Bob", "Carol"]), Some(mock_user(1, "Voter1"))),
            (mock_vote(2, 1, 2, vec!["Bob", "Alice", "Carol"]), Some(mock_user(2, "Voter2"))),
            (mock_vote(3, 1, 3, vec!["Alice", "Carol", "Bob"]), Some(mock_user(3, "Voter3"))),
            (mock_vote(4, 1, 4, vec!["Carol", "Alice", "Bob"]), Some(mock_user(4, "Voter4"))),
        ];
        let mut event = mock_event(votes);
        event.event_type = EventType::Election;
        let bytes = event.export_result_pdf();
        std::fs::write("/tmp/test_event_result_election.pdf", &bytes).expect("Failed to write PDF");
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
