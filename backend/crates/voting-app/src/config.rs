use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_base_url: String,
    pub database_url: String,
    pub oidc_issuer: String,
    pub oidc_client_id: String,
    pub oidc_client_secret: String,
    pub bind_addr: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Self {
            app_base_url: must_env("APP_BASE_URL")?,
            database_url: must_env("DATABASE_URL")?,
            oidc_issuer: must_env("OIDC_ISSUER")?,
            oidc_client_id: must_env("OIDC_CLIENT_ID")?,
            oidc_client_secret: must_env("OIDC_CLIENT_SECRET")?,
            bind_addr: env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
        })
    }
}

fn must_env(name: &str) -> Result<String, String> {
    env::var(name).map_err(|_| format!("{name} must be set"))
}
