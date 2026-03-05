use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::middleware::Next;

use crate::infrastructure::services::auth::validate_token;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    // Let CORS preflight requests pass through
    if req.method() == actix_web::http::Method::OPTIONS {
        return next.call(req).await;
    }

    let path = req.path().to_string();

    // Public endpoints
    if path == "/api/auth/login" || path == "/api/config" {
        return next.call(req).await;
    }

    // Only protect /api/* routes
    if !path.starts_with("/api/") {
        return next.call(req).await;
    }

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(String::from);

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];
            match validate_token(token) {
                Ok(_) => next.call(req).await,
                Err(_) => Err(ErrorUnauthorized("Invalid or expired token")),
            }
        }
        _ => Err(ErrorUnauthorized("Missing authorization token")),
    }
}
