use crate::initialize::initialize_app;
use shared::configs::APP_CONFIG;
use tokio::signal;

mod initialize;

#[tokio::main]
async fn main() {
    let app = initialize_app().await.expect("Error initializing app");

    let host = APP_CONFIG.server.host.clone();
    let port = APP_CONFIG.server.port;
    let address = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.expect("Failed to start server 🐧");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("🛑 Shutting down gracefully...");
}
