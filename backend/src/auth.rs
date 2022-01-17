use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct IndexResponse {
    user_id: Option<String>,
    counter: i32,
}

#[derive(Deserialize)]
struct Identity {
    user_id: String,
}

#[post("/login")]
async fn login(user_id: web::Json<Identity>, session: Session) -> impl Responder {
    let id = user_id.into_inner().user_id;
    // TODO: delete unwrap
    session.set("user_id", &id).unwrap();
    session.renew();

    let counter = session
        .get::<i32>("counter")
        .unwrap_or(Some(0))
        .unwrap_or(0);

    HttpResponse::Ok().json(IndexResponse {
        user_id: Some(id),
        counter,
    })
}
