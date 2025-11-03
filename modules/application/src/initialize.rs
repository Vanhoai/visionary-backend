use adapters::primary::routes;
use axum::Router;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderName, HeaderValue, Method};
use shared::configs::APP_CONFIG;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

use adapters::shared::di::state::AppState;

fn allow_method_from_string(method: &str) -> Result<Method, Box<dyn std::error::Error>> {
    match method.to_uppercase().as_str() {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "PATCH" => Ok(Method::PATCH),
        "DELETE" => Ok(Method::DELETE),
        "PUT" => Ok(Method::PUT),
        "OPTIONS" => Ok(Method::OPTIONS),
        "HEAD" => Ok(Method::HEAD),
        "TRACE" => Ok(Method::TRACE),
        "CONNECT" => Ok(Method::CONNECT),
        _ => Err(format!("Unsupported HTTP method: {}", method).into()),
    }
}

fn build_cors() -> Result<CorsLayer, Box<dyn std::error::Error>> {
    let origins = APP_CONFIG
        .cors
        .allow_origins
        .iter()
        .map(|origin| HeaderValue::from_str(origin))
        .collect::<Result<Vec<HeaderValue>, _>>()?;

    let allow_methods = APP_CONFIG
        .cors
        .allow_methods
        .iter()
        .map(|method| allow_method_from_string(method))
        .collect::<Result<Vec<Method>, _>>()?;

    let allow_headers = APP_CONFIG
        .cors
        .allow_headers
        .iter()
        .map(|header| header.parse::<HeaderName>().unwrap())
        .collect::<Vec<HeaderName>>();

    Ok(CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(APP_CONFIG.cors.allow_credentials)
        .allow_methods(allow_methods)
        .allow_headers(allow_headers))
}

pub async fn initialize_app() -> Result<Router, Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_max_level(Level::INFO)
        .with_line_number(true)
        .with_level(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    tracing::info!("🔧 Mode: {}", APP_CONFIG.mode);
    tracing::info!("🦀 Server: {}:{}", APP_CONFIG.server.host, APP_CONFIG.server.port);

    let traces = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    let state = Arc::new(AppState::new().await);
    match APP_CONFIG.cors.enabled {
        true => {
            tracing::info!("🌐 CORS is enabled");
            let cors = build_cors()?;
            Ok(routes::execute().layer(cors).layer(traces).with_state(state.clone()))
        },

        false => {
            tracing::info!("🌐 CORS is disabled");
            Ok(routes::execute().layer(traces).with_state(state))
        },
    }
}
