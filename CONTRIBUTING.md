# Contributing
Before contributing to this repository, please discuss the change you wish to make via issue on this repository, email to one of the codeowners, or on the ScottyLabs [discord](go.scottylabs.org/discord).

## Extensive Guide to Running Voting App

### Prerequisites

First, you need Bun, Cargo, and Docker (Desktop optional) installed.

To install Bun, follow the documentation found at [https://bun.com/docs/installation], or run:
`curl -fsSL https://bun.com/install | bash`

To install Cargo, similarly, follow the documentation at [https://doc.rust-lang.org/cargo/getting-started/installation.html] or run:
`curl https://sh.rustup.rs -sSf | sh`

For Docker, you may optionally use Docker Desktop, providing a GUI interface, which can be installed from [https://www.docker.com/get-started/]. On the otherhand, you may want to only install Docker Enginer, which includes a server and a CLI, found at [https://docs.docker.com/engine/install].

### Starting up
Now, we will get your own instance of Voting App running!

#### Setup
You will need [git](https://git-scm.com/install/).

Run `git clone https://github.com/ScottyLabs/voting-app.git` in your favorite (or least favorite) folder to download the repository, and run `cd voting-app` to enter.

#### Backend
To start the backend, first navigate with

```cd backend```

Next, to set up the PostgreSQL database inside of Docker,

```backend $ docker compose up -d```

You should see a message similar to `Container backend-db-1 Started`.

To start the backend, navigate to the crate which holds the main files with

```cd crates/voting-app```

Now, build and run the backend with

```backend/crates/voting-app $ cargo run```

Eventually (probably not now), you may want to stop Docker. Run (in the backend folder)

```backend $ docker compose down```

#### Frontend
Now that your backend is running, we can set up the frontend. Navigate to the frontend folder. You will probably need another terminal instance, because you need both running at the same time. Navigate with

```cd frontend```

We want to install the proper dependencies for the frontend. Run

```frontend $ bun install```

followed by

```frontend $ bun run dev```

to start up the frontend.
