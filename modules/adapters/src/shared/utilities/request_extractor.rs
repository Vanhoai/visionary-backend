use crate::shared::models::failure::HttpFailure;
use axum::extract::{FromRequest, Request};
use axum::http::HeaderMap;

pub struct RequestMetadata {
    pub id_address: String,
    pub user_agent: String,
    pub device_type: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RequestExtractor<T>(pub T);

impl<S> FromRequest<S> for RequestExtractor<RequestMetadata>
where
    S: Send + Sync,
{
    type Rejection = HttpFailure;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let header = req.headers();
        let ip_address = extract_ip(&header);
        let user_agent = extract_user_agent(&header);
        let device_type = detect_device_type(&user_agent);

        let metadata = RequestMetadata { id_address: ip_address, user_agent, device_type };
        Ok(RequestExtractor(metadata))
    }
}

pub fn extract_ip(headers: &HeaderMap) -> String {
    // Try to get IP from X-Forwarded-For header (for proxied requests)
    if let Some(forwarded_for) = headers.get("x-forwarded-for") {
        if let Ok(value) = forwarded_for.to_str() {
            // Take the first IP if there are multiple
            if let Some(ip) = value.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }

    // Try X-Real-IP header
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(value) = real_ip.to_str() {
            return value.to_string();
        }
    }

    // Fallback to unknown
    "unknown".to_string()
}

pub fn extract_user_agent(headers: &HeaderMap) -> String {
    headers.get("user-agent").and_then(|h| h.to_str().ok()).unwrap_or("unknown").to_string()
}

pub fn detect_device_type(user_agent: &str) -> String {
    let ua_lower = user_agent.to_lowercase();

    if ua_lower.contains("mobile") || ua_lower.contains("android") || ua_lower.contains("iphone") {
        "mobile".to_string()
    } else if ua_lower.contains("tablet") || ua_lower.contains("ipad") {
        "tablet".to_string()
    } else if ua_lower.contains("bot") || ua_lower.contains("crawler") || ua_lower.contains("spider") {
        "bot".to_string()
    } else {
        "desktop".to_string()
    }
}
