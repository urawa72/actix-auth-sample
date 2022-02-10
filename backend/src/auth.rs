use actix_session::Session;
use actix_web::{post, web, HttpResponse, Result};
use actix_web_grants::proc_macro::has_any_permission;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndexResponse {
    counter: i32,
}

#[derive(Deserialize, Debug)]
struct Identity {
    user_id: String,
}

#[post("/do_something")]
#[has_any_permission("Write")]
async fn do_something(session: Session) -> Result<HttpResponse> {
    let counter: i32 = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .map_or(1, |inner| inner + 1);
    session.set("counter", counter)?;

    Ok(HttpResponse::Ok().json(IndexResponse { counter }))
}

#[post("/login")]
async fn login(identity: web::Json<Identity>, session: Session) -> Result<HttpResponse> {
    let id = identity.into_inner().user_id;
    session.set("user_id", &id)?;
    session.renew();

    let counter = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .unwrap_or(0);

    Ok(HttpResponse::Ok().json(IndexResponse { counter }))
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
