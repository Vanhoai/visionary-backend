use std::sync::Arc;
use axum::Router;
use crate::shared::di::state::AppState;

pub mod v1;

pub fn execute() -> Router<Arc<AppState>> {
    let v1 = Router::new()
        .nest("/auth", v1::auth::routes::execute())
        .nest("/accounts", v1::accounts::routes::execute());

    Router::new().nest("/api/v1", v1)
}
