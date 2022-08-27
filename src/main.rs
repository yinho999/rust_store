extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::http::header;
use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use csrf::AesGcmCsrfProtection;
use db_connection::establish_connection;

use std::sync::Mutex;
pub mod db_connection;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod utils;

async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // init env_logger
    env_logger::init();
    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();
    let csrf_token_header = header::HeaderName::from_lowercase(b"x-csrf-token").unwrap();
    let csrf_token_cookie_header =
        header::HeaderName::from_lowercase(b"x-csrf-token-cookie").unwrap();

    let generator = AesGcmCsrfProtection::from_key(*b"01234567012345670123456701234567");
    let wrapped_generator = web::Data::new(Mutex::new(generator));

    let pool = Data::new(establish_connection());
    // Create an instance of the server.
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("redis://127.0.0.")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::ACCEPT,
                csrf_token_header.clone(),
                csrf_token_cookie_header.clone(),
            ])
            .expose_headers(vec![
                csrf_token_header.clone(),
                csrf_token_cookie_header.clone(),
            ])
            .max_age(3600);
        // Create an instance of the app.
        App::new()
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(cors)
            .app_data(Data::clone(&wrapped_generator))
            .app_data(pool.clone())
            .route("/", web::get().to(index))
            // Route the index function to the root path.
            .service(
                web::scope("/products")
                    .service(handlers::products::index)
                    .service(handlers::products::get)
                    .service(handlers::products::update)
                    .service(handlers::products::create)
                    .service(handlers::products::destroy),
            )
            .service(
                web::scope("/auth")
                    .service(handlers::authentication::login)
                    .service(handlers::authentication::logout),
            )
    })
    // bind to port 8080 and run the server on "127.0.0.1"
    .bind(("127.0.0.1", 8088))?
    .run()
    .await
}
