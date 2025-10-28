use axum::Router;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

use core::configs::APP_CONFIG;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt().with_target(false).compact().init();
    tracing::info!("Starting application with mode: {}", APP_CONFIG.mode);

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let traces = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let app = Router::new().layer(cors).layer(traces);

    let host = APP_CONFIG.server.host.clone();
    let port = APP_CONFIG.server.port;
    let address = format!("{}:{}", host, port);

    tracing::info!("listening on {} 🎉", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
