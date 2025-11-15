use axum::extract::{FromRequestParts, Request};
use axum::http::HeaderMap;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;

// shared modules
use shared::jwt::service::JwtService;
use shared::models::failure::Failure;

// internal modules
use crate::shared::models::failure::HttpFailure;

#[derive(Debug, Clone)]
pub struct AuthClaims {
    pub account_id: String,
    pub jti: String,
    pub role: Option<String>,
}

impl AuthClaims {
    pub fn from_headers(headers: &HeaderMap) -> Result<Self, HttpFailure> {
        let token = headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .and_then(|auth| auth.strip_prefix("Bearer "))
            .ok_or_else(|| {
                HttpFailure::new(Failure::Unauthorized("Missing or invalid Authorization header".to_string()))
            })?;

        let claims_wrapped = JwtService::verify_access_token(token).map_err(HttpFailure::new)?;
        Ok(AuthClaims {
            account_id: claims_wrapped.claims.sub,
            jti: claims_wrapped.claims.jti,
            role: claims_wrapped.claims.role,
        })
    }
}

impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = HttpFailure;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<AuthClaims>().cloned().ok_or_else(|| {
            HttpFailure::new(Failure::Unauthorized(
                "Authentication required. Please ensure auth_middleware is applied.".to_string(),
            ))
        })
    }
}

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, HttpFailure> {
    let headers = req.headers();
    let claims = AuthClaims::from_headers(headers)?;

    // Store claims in request extensions for later use
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}
