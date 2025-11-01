use adapters::primary::routes;
use axum::Router;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderName, HeaderValue, Method};
use shared::configs::APP_CONFIG;
use tower_http::cors::CorsLayer;
use tower_http::trace;
use tower_http::trace::TraceLayer;
use tracing::Level;

fn allow_method_from_string(method: &str) -> Method {
    match method.to_uppercase().as_str() {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PATCH" => Method::PATCH,
        "DELETE" => Method::DELETE,
        "PUT" => Method::PUT,
        "OPTIONS" => Method::OPTIONS,
        "HEAD" => Method::HEAD,
        "TRACE" => Method::TRACE,
        "CONNECT" => Method::CONNECT,
        _ => Method::GET,
    }
}

fn build_cors() -> CorsLayer {
    let origins = APP_CONFIG
        .cors
        .allow_origins
        .iter()
        .map(|origin| HeaderValue::from_str(origin).unwrap())
        .collect::<Vec<HeaderValue>>();

    let allow_methods =
        APP_CONFIG.cors.allow_methods.iter().map(|method| allow_method_from_string(method)).collect::<Vec<Method>>();

    let allow_headers = APP_CONFIG
        .cors
        .allow_headers
        .iter()
        .map(|header| header.parse::<HeaderName>().unwrap())
        .collect::<Vec<HeaderName>>();

    CorsLayer::new()
        .allow_origin(origins)
        .allow_credentials(APP_CONFIG.cors.allow_credentials)
        .allow_methods(allow_methods)
        .allow_headers(allow_headers)
}

pub fn initialize_app() -> Router {
    tracing_subscriber::fmt().with_target(false).compact().init();
    tracing::info!("Starting application with mode: {:?}", APP_CONFIG.mode);

    let traces = TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO));

    match APP_CONFIG.cors.enabled {
        true => {
            tracing::info!("CORS is enabled");
            routes::execute().layer(build_cors()).layer(traces)
        },

        false => {
            tracing::info!("CORS is disabled");
            routes::execute().layer(traces)
        },
    }
}
