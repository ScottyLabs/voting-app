# Schema Information

* [Users](db-schema.md#users)
* [Organizations](db-schema.md#organizations)
* [Organization Members](db-schema.md#organization-members)
* [Events](db-schema.md#events)
* [Votes](db-schema.md#votes)

## Users

### Schema Definition
```rust
struct User {
  id: i32,
  name: string,
  created_at: DateTimeWithTimeZone
}
```

## Organizations
```rust
struct Organization {
  id: i32,
  name: string,
  data: JSON
}
```

## Organization Members
```rust
struct OrganizationMember {
  organization_id: i32,
  user_id: i32,
  user_role: string,
  joined_at: DateTimeWithTimeZone
}
```

## Events
```rust
struct Event {
  id: i32,
  event_type: String,
  name: String,
  status: String,
  start_time: DateTimeWithTimeZone,
  end_time: Option<DateTimeWithTimeZone>,
  data: Json,
  created_by_user_id: i32,
  organization_id: i32,
}
```

## Votes
```rust
struct Vote {
  id: i32,
  event_id: i32,
  voter_id: i32,
  cast_time: DateTimeWithTimeZone,
  proxy: bool,
  data: Json,
}
```
