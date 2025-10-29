use axum::Router;

pub mod v1;

pub fn execute() -> Router {
    let v1 = Router::new()
        .nest("/auth", v1::auth::routes::execute())
        .nest("/accounts", v1::accounts::routes::execute());

    Router::new().nest("/api/v1", v1)
}
