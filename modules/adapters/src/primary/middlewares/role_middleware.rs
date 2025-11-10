use crate::primary::middlewares::auth_middleware::AuthClaims;
use crate::shared::models::failure::HttpFailure;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use shared::models::failure::Failure;

pub async fn role_middleware(required_roles: Vec<String>, req: Request, next: Next) -> Result<Response, HttpFailure> {
    let claims = req.extensions().get::<AuthClaims>().cloned().ok_or_else(|| {
        HttpFailure::new(Failure::Unauthorized(
            "Authentication required. Please ensure auth_middleware is applied first.".to_string(),
        ))
    })?;

    if !required_roles.is_empty() {
        if claims.role.is_none() {
            return Err(HttpFailure::new(Failure::Forbidden("Access denied: No role assigned".to_string())));
        }

        let account_role = claims.role.clone().unwrap();
        if !required_roles.contains(&account_role) {
            return Err(HttpFailure::new(Failure::Forbidden(format!(
                "Access denied. Required roles: {:?}, your role: {}",
                required_roles, account_role
            ))));
        }

        println!("Access granted for role: {}", account_role);
    }

    Ok(next.run(req).await)
}

pub fn require_roles(
    roles: Vec<&str>,
) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn Future<Output = Result<Response, HttpFailure>> + Send>> + Clone {
    let required_roles: Vec<String> = roles.iter().map(|r| r.to_string()).collect();
    move |req: Request, next: Next| {
        let roles = required_roles.clone();
        Box::pin(async move { role_middleware(roles, req, next).await })
    }
}

// Helper function: Require ADMIN role only
pub async fn require_admin(req: Request, next: Next) -> Result<Response, HttpFailure> {
    role_middleware(vec!["ADMIN".to_string()], req, next).await
}

// Helper function: Require NORMAL role only
pub async fn require_normal(req: Request, next: Next) -> Result<Response, HttpFailure> {
    role_middleware(vec!["NORMAL".to_string()], req, next).await
}
