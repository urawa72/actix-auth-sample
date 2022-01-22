use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_web::{http::header, middleware::Logger, App, HttpServer};

mod auth;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // std::env::set_var("RUST_LOG", "actix=info");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(move || {
        // Cors settings
        let cors = Cors::default()
            .allowed_origin("http://sub.localhost:4000")
            .allowed_origin("http://localhost:4000")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(
                RedisSession::new("127.0.0.1:6379", &[0u8; 32]) //TODO: use random key from env file
                    .cookie_http_only(true)
                    .cookie_secure(false)
                    // .cookie_same_site(actix_redis::SameSite::None)
                    .cookie_domain("localhost")
                    .cookie_name("actix-auth-sample"),
            )
            .wrap(Logger::default())
            .service(auth::login)
            .service(auth::logout)
            .service(auth::do_something)
            .service(user::get_users_info)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
