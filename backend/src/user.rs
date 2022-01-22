use actix_session::Session;
use actix_web::{get, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    username: String,
    email: String,
}

#[get("/users/info")]
pub async fn get_users_info(session: Session) -> Result<HttpResponse> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    Ok(HttpResponse::Ok().json(UserInfo {
        username: "test".to_string(),
        email: "test@example.com".to_string()
    }) )
}
