use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_web::dev::ServiceRequest;
use actix_web::{http::header, middleware::Logger, App, HttpServer};
use actix_web_grants::GrantsMiddleware;

mod auth;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    println!("Listening on: 127.0.0.1:8000");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://sub.localhost.test:4000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::ACCEPT, header::CONTENT_TYPE])
            .max_age(60)
            .supports_credentials();

        let session = RedisSession::new("127.0.0.1:6379", &[0u8; 32]) // TODO: use random key from env file
            .cookie_http_only(true)
            .cookie_secure(false) // only for local
            .cookie_domain("localhost.test")
            .cookie_name("actix-auth-sample");

        App::new()
            .wrap(GrantsMiddleware::with_extractor(extract))
            .wrap(middleware::AuthService)
            .wrap(session)
            .wrap(cors)
            .wrap(Logger::default())
            .service(auth::login)
            .service(auth::logout)
            .service(auth::do_something)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

async fn extract(_req: &mut ServiceRequest) -> Result<Vec<String>, actix_web::Error> {
    Ok(vec![
        "Read".to_string(),
        "Write".to_string(),
        "Piyo".to_string(),
    ])
}
