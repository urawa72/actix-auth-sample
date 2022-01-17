use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .supports_credentials()
            .allowed_origin("http://localhost:4000")
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec!["X-CUSTOM-HEADER"]);
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(user::get_info)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
