use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpServer};

mod user;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::default().allowed_origin("http://localhost:4000"))
            .wrap(Logger::default())
            .service(user::get_info)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
