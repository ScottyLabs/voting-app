use std::env;

use axum::Router;
use dotenvy::{dotenv, from_filename};

mod event;
mod log;
mod organization;
mod user;
mod vote;

use crate::{
    event::router::router as event_router, log::router::router as log_router,
    organization::router::router as organization_router, user::router::router as user_router,
    vote::router::router as vote_router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Try backend/.env first, then fall back to voting-app/.env.
    let _ = dotenv();
    if env::var("DATABASE_URL").is_err() {
        let _ = from_filename("../.env");
    }

    let backend_bind_address = env::var("BACKEND_URL").unwrap_or_else(|_| {
        let backend_bind_address_placeholder = "127.0.0.1:3000";
        backend_bind_address_placeholder.to_owned()
    });

    let vote_routes = vote_router().await?;
    let organization_routes = organization_router().await?;
    let user_routes = user_router().await?;
    let event_routes = event_router().await?;
    let log_routes = log_router().await?;
    let app = Router::new()
        .nest("/api/vote", vote_routes)
        .nest("/api/organization", organization_routes)
        .nest("/api/user", user_routes)
        .nest("/api/event", event_routes)
        .nest("/api/log", log_routes);

    let listener = tokio::net::TcpListener::bind(&backend_bind_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
