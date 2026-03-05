use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub email: String,
}
