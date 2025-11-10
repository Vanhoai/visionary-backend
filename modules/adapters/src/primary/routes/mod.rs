use crate::shared::di::state::AppState;
use axum::Router;
use std::sync::Arc;

pub mod v1;

pub fn execute() -> Router<Arc<AppState>> {
    let v1 = Router::new()
        .nest("/auth", v1::auth::routes::execute())
        .nest("/accounts", v1::accounts::routes::execute())
        .nest("/sessions", v1::sessions::routes::execute())
        .nest("/experiences", v1::experiences::routes::execute())
        .nest("/blogs", v1::blogs::routes::execute())
        .nest("/categories", v1::categories::routes::execute());

    Router::new().nest("/api/v1", v1)
}
