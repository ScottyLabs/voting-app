# Voting App

Voting App is a voting app that does voting app things.

## Getting Started

### Prerequisites

- Bun - Javascript runtime and package manager
- Docker - For running PostgreSQL database

### Setup

For detailed setup instructions, such as configuring your development, see CONTRIBUTING.md

**Quick setup:**
1. Install Bun and Docker (see links above)

### Starting the backend
```bash
# 1. Go to backend folder
cd backend

# 2. Start Docker
docker compose up -d

# 3. Go into voting-app folder
cd crates/voting-app

# 3. Start the backend
cargo run

# 4. Later, to quit docker, go to the backend folder and run
docker compose down
```

### Starting the frontend
```bash
# 1. Go to frontend folder
cd frontend

# 2. Install dependencies
bun install

# 3. Start the frontend
bun run dev
```

### Contributing

Please check [CONTRIBUTING.md](CONTRIBUTING.md) before you contribute to this project!
