# Auth

This is a quick guide to the auth files in `backend/crates/voting-app/src`, which mirrors [Terrier](https://github.com/ScottyLabs/terrier-old/):

- `OidcAuthLayer` loads auth/session claims.
- `OidcLoginLayer` requires login on selected routes.
- User sync middleware maps OIDC identity to a local DB user.
- Handlers serve login/callback/logout/status routes.

## `src/server.rs`

Builds the server, middleware stack, and routes.

1. Loads config from env.
2. Connects to DB.
3. Runs migrations at startup.
4. Creates the session layer (`tower-sessions` memory store).
5. Discovers OIDC provider and creates `OidcAuthLayer`.
6. Creates:
   - `oidc_auth_service` (token/session validation)
   - `oidc_login_service` (forces login redirects)
7. Builds routers:
   - Protected auth router (`/auth/callback`, `/auth/logout`)
   - Public routes (`/`, `/auth/login`, `/auth/status`, `/health`)
8. Starts the server.

- `OidcAuthLayer` is global.
- `OidcLoginLayer` is only on protected routes.
- `sync_user_middleware` runs on protected routes.

## `src/core/auth/middleware.rs`

Syncs authenticated OIDC users into local DB users.

- `SyncedUser(pub Arc<user::Model>)`
  - Request extension for the local user model.
  - Supports required and optional extractors.

- `sync_user_middleware(...)`
  - Reads `OidcClaims`.
  - Finds user by `user.name == oidc_sub`.
  - Creates user if missing.
  - Inserts `SyncedUser` into request extensions.

## `src/domain/auth/handlers.rs`

Auth handlers for these endpoints:

- `GET /auth/login`
  - Starts login.
  - Redirects to `/auth/callback`.

- `GET /auth/callback`
  - Authenticated callback route.
  - Redirects to validated `redirect_uri` or `APP_BASE_URL`.

- `GET /auth/logout`
  - Uses `OidcRpInitiatedLogout`.
  - Sets post-logout redirect to `APP_BASE_URL`.

- `GET /auth/status`
  - Returns JSON auth state.
