use actix_web::{post, web::Json, HttpResponse};

use crate::application::dto::auth_dto::LoginResponse;
use crate::application::request_dto::login_dto::LoginRequest;
use crate::infrastructure::repositories::user::UserRepository;
use crate::infrastructure::services::auth::create_token;

#[post("/api/auth/login")]
pub async fn login(body: Json<LoginRequest>) -> HttpResponse {
    let params = body.into_inner();

    let user_repo = UserRepository::new().await;

    let user = match user_repo.find_by_email(&params.email).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid email or password"
            }));
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Internal server error"
            }));
        }
    };

    let valid = bcrypt::verify(&params.password, user.get_password_hash()).unwrap_or(false);
    if !valid {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid email or password"
        }));
    }

    match create_token(&user.get_id().to_hex(), user.get_email()) {
        Ok(token) => HttpResponse::Ok().json(LoginResponse {
            token,
            email: user.get_email().to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Failed to generate token"
        })),
    }
}
