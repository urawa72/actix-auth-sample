use actix_cors::Cors;
use actix_redis::RedisSession;
use actix_web::{middleware::Logger, App, HttpServer};
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

mod auth;
mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    // Generate a random 32 byte key
    let mut csp_rng = ChaCha20Rng::from_entropy();
    let mut private_key = [0u8; 32];
    csp_rng.fill_bytes(&mut private_key);

    HttpServer::new(move || {
        // Cors settings
        let cors = Cors::default()
            .supports_credentials()
            .allowed_origin("http://localhost:4000")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec!["X-CUSTOM-HEADER"]);

        App::new()
            .wrap(cors)
            .wrap(RedisSession::new("127.0.0.1:6379", &private_key))
            .wrap(Logger::default())
            .service(user::get_info)
            .service(auth::login)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
