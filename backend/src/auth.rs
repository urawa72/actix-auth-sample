use actix_session::Session;
use actix_web::{post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndexResponse {
    user_id: Option<String>,
    counter: i32,
}

#[derive(Deserialize, Debug)]
struct Identity {
    user_id: String,
}

#[post("/do_something")]
async fn do_something(session: Session) -> Result<HttpResponse> {
    if session.get::<String>("user_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let user_id: Option<String> = session.get::<String>("user_id")?;
    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .unwrap_or(0);
    Ok(HttpResponse::Ok().json(IndexResponse { user_id, counter }))
}

#[post("/login")]
async fn login(user_id: web::Json<Identity>, session: Session) -> Result<HttpResponse> {
    let id = user_id.into_inner().user_id;
    session.set("user_id", &id)?;
    session.renew();

    let counter = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .unwrap_or(0);

    Ok(HttpResponse::Ok().json(IndexResponse {
        user_id: Some(id),
        counter,
    }))
}

#[post("/logout")]
async fn logout(session: Session) -> Result<HttpResponse> {
    let user_id = session.get::<String>("user_id")?;
    if let Some(x) = user_id {
        session.purge();
        Ok(format!("Logged out: {}", x).into())
    } else {
        Ok("Could not log out anonymous user".into())
    }
}