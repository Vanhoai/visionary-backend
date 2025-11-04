use crate::shared::di::state::AppState;
use axum::Router;
use std::sync::Arc;

pub mod v1;

pub fn execute() -> Router<Arc<AppState>> {
    let v1 = Router::new()
        .nest("/auth", v1::auth::routes::execute())
        .nest("/accounts", v1::accounts::routes::execute())
        .nest("/sessions", v1::sessions::routes::execute());

    Router::new().nest("/api/v1", v1)
}
