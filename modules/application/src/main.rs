use crate::initialize::initialize_app;
use shared::configs::APP_CONFIG;

mod initialize;

#[tokio::main]
async fn main() {
    let app = initialize_app();

    let host = APP_CONFIG.server.host.clone();
    let port = APP_CONFIG.server.port;
    let address = format!("{}:{}", host, port);

    tracing::info!("listening on {} 🎉", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
