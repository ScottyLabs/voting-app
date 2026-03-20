# JSON Information

* [vote.data](db-json.md#votedata)
* [event.data](db-json.md#eventdata)
* [organization.data](db-json.md#organizationdata)
* [log.data](db-json.md#logdata)

## `vote.data`

The `vote.data` field stores the submitted voting payload for a single vote record.

### Type Definition
```typescript
type VoteData = {
  vote_type: string,
  vote_response: string[]
}
```

### Field Definitions
- `vote_type`: Specifies the type of vote represented by this response. Examples include `"motion"` and `"election"`.
- `vote_response`: Stores the participant’s submitted response as an array.
  - For a standard motion, this array typically contains a single value.
  - For a ranked-choice election, this array stores the ranked selections in order of preference.
  - A lower array index indicates a higher preference (e.g., index 0 = first choice, index 1 = second choice).

### Example Usage
```json
{
  "vote_type": "motion",
  "vote_response": ["choice1", "choice2"]
}
```

## `event.data`

The `event.data` field stores event-specific configuration and metadata.

### Type Definition
```typescript
type EventData = {
  description: string,
  session_code: string,
  vote_type: string,
  threshold: number,
  visibility: {
    participants: string
  },
  proxy: bool,
  vote_options: string[]
}
```

### Field Definitions
- `description`: A textual description of the event.
- `session_code`: A code used for joining or identifying the session.
- `vote_type`: Specifies whether the event is a motion or an election.
- `threshold`: A floating-point value representing the approval threshold required for the vote. This value in the range [0, 1].
- `visibility.participants`: Defines what participants can see during the event. Example values include `"hidden_until_release"` and `"live"`.
- `proxy`: Indicates whether proxy voting is enabled for the event.
- `vote_options`: Lists the selectable voting options for the event.

### Example Usage
```json
{
  "description": "Event description goes here",
  "session_code": "code",
  "vote_type": "motion",
  "threshold": 0.75,
  "visibility": {
    "participants": "hidden_until_release"
  },
  "proxy": true,
  "vote_options": ["option1", "option2"]
}
```

### Notes
- For now, all users are treated as having the same role.

## `organization.data`

The `organization.data` field stores organization-level metadata.

### Type Definition
```typescript
type OrganizationData = {
  description: string
}
```

### Field Definitions
- description: A textual description of the organization.

### Example Usage
```json
{
  "description": "Organization Description"
}
```

## `log.data`

The `log.data` field stores audit log information for system actions.

### Type Definition
```typescript
type LogData = {
  action: string,
  target: {
    table: string,
    id: number
  },
  actor: {
    user_id: number,
    role: string
  } | "system",
  event_id: number,
  changes: {
    before: {},
    after: {}
  }
}
```

### Field Definitions
- `action`: A description of the action being logged.
- `target`: Identifies the record affected by the action.
- `table`: The name of the affected table.
- `id`: The primary key of the affected record.
- `actor`: Identifies the user responsible for the action.
- `user_id`: The ID of the acting user.
- `role`: The role of the acting user.
- `event_id`: The related event identifier, if applicable.
- `changes`: Stores the state transition caused by the action.
- `before`: The state before the change.
- `after`: The state after the change.

### Example Usage
```json
{
  "action": "Action Description",
  "target": {
    "table": "tablename",
    "id": 0
  },
  "actor": {
    "user_id": 0,
    "role": "Role String"
  },
  "event_id": 0,
  "changes": {
    "before": {},
    "after": {}
  }
}
```
