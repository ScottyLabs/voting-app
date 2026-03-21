# Voting App

Voting App is a CMU Undergraduate Senate-commissioned, ScottyLabs-developed voting app, to help the Senate and other student organizations manage attendance and host elections and motions. Currently, the app is still under development, but we *strongly* hope to get it completed very soon! <!-- Add information about where to access the website here, when the MVP is done. -->

### Built With
- Svelte
- Rust
- PostgreSQL

## Assumptions about the reader
Hello, reader! For the remainder of this README, and other documentation, we will assume that you are a developer or contributor, using WSL or a Unix development system, and have some familiarity with the command line. If you need any help, you are free to contact one of the codeowners found in .github/CODEOWNERS, or join the [discord](https://go.scottylabs.org/discord).

## Getting Started

### Prerequisites

- [Bun](https://bun.com/docs/installation) - Javascript runtime and package manager
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) - Rust package manager and build system
- [Docker](https://www.docker.com/get-started/) - For running the PostgreSQL database

### Quick Setup

For detailed setup instructions, see [SETUP.md](docs/SETUP.md).

Install Bun, Cargo, and Docker (see links above).

#### Starting the backend
```bash
# Copy the .env.example
$ cp .env.example .env

# Start Docker
backend $ docker compose up -d

# Run the backend
backend/crates/voting-app $ cargo run

# Stop Docker
backend $ docker compose down
```

#### Starting the frontend

```bash
# Install dependencies
frontend $ bun install

# 3. Start the frontend
frontend $ bun run dev
```

### Contributing

Please check [CONTRIBUTING.md](docs/CONTRIBUTING.md) before you contribute to this project!

### Licenses
Voting App is distributed under the Apache 2.0 and MIT Licenses, found in the files `LICENSE-APACHE-2.0` and `LICENSE-MIT` respectively.
