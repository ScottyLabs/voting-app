use std::env;

use axum::Router;
use dotenvy::{dotenv, from_filename};

mod organization;
mod vote;

use crate::{
    organization::router::router as organization_router, vote::router::router as voting_router,
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

    let voting_routes = voting_router().await?;
    let organization_routes = organization_router().await?;
    let app = Router::new()
        .nest("/api/vote", voting_routes)
        .nest("/api/organization", organization_routes);

    let listener = tokio::net::TcpListener::bind(&backend_bind_address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
