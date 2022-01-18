use actix_web::{get, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    username: String,
    email: String,
}

#[get("/users/info")]
pub async fn get_users_info() -> web::Json<UserInfo> {
    web::Json(UserInfo {
        username: "test".to_string(),
        email: "test@example.com".to_string()
    })
}
