use actix_web::{get, HttpResponse};

use crate::application::dto::config_dto::ConfigDto;

#[get("/api/config")]
pub async fn get_config() -> HttpResponse {
    HttpResponse::Ok().json(ConfigDto::default_config())
}
