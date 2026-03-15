# Database Information

## `vote.response` (JSON)

The `vote.response` field stores the submitted voting payload for a single vote record.

### Structure
```json
{
  "vote_type": "motion",
  "vote_response": ["response1", "response2"]
}
```
Field Definitions
	•	vote_type: Specifies the type of vote represented by this response. Examples include "motion" and "election".
	•	vote_response: Stores the participant’s submitted response as an array.
	•	For a standard motion, this array typically contains a single value.
	•	For a ranked-choice election, this array stores the ranked selections in order of preference.
	•	In ranked voting, a lower array index indicates a higher preference. For example, index 0 represents the first choice, index 1 represents the second choice, and so on.

⸻

event.data (JSON)

The event.data field stores event-specific configuration and metadata.

Notes
	•	Attendance should be represented as a dictionary mapping user.user_id to a boolean value.
	•	For now, all users are treated as having the same role.

Structure
```json
{
  "description": "event description goes here",
  "session_code": "code",
  "vote_type": "motion or election",
  "threshold": 0.75,
  "visibility": {
    "participants": "hidden_until_release/live"
  },
  "proxy": true,
  "vote_options": ["option1", "option2"]
}
```
Field Definitions
	•	description: A textual description of the event.
	•	session_code: A code used for joining or identifying the session.
	•	vote_type: Specifies whether the event is a motion or an election.
	•	threshold: A floating-point value representing the approval threshold required for the vote.
	•	visibility.participants: Defines what participants can see during the event. Example values include:
	•	hidden_until_release
	•	live
	•	proxy: Indicates whether proxy voting is enabled for the event.
	•	vote_options: Lists the selectable voting options for the event.

Attendance Representation

Example attendance dictionary:
```json
{
  "0000": true,
  "1111": false,
  "2222": true
}
```
In this representation:
	•	the key is user.user_id
	•	the value is a boolean indicating whether the user is marked present

⸻

organization.data (JSON)

The organization.data field stores organization-level metadata.

Structure
```json
{
  "description": "description"
}
```
Field Definitions
	•	description: A textual description of the organization.

⸻

log.data (JSON)

The log.data field stores audit log information for system actions.

Structure
```json
{
  "action": "action description",
  "target": {
    "table": "tablename",
    "id": 0
  },
  "actor": {
    "user_id": 0,
    "role": "role string"
  },
  "event_id": 0,
  "changes": {
    "before": {},
    "after": {}
  }
}
```
Field Definitions
	•	action: A description of the action being logged.
	•	target: Identifies the record affected by the action.
	•	table: The name of the affected table.
	•	id: The primary key of the affected record.
	•	actor: Identifies the user responsible for the action.
	•	user_id: The ID of the acting user.
	•	role: The role of the acting user.
	•	event_id: The related event identifier, if applicable.
	•	changes: Stores the state transition caused by the action.
	•	before: The state before the change.
	•	after: The state after the change.

