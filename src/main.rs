extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web::{self, Data}, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware::Logger};
use db_connection::establish_connection;
pub mod db_connection;
pub mod models;
pub mod schema;
pub mod handlers; 
pub mod errors;

async fn index(_req : HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    // init env_logger
    env_logger::init();

    let pool = Data::new(establish_connection());
    // Create an instance of the server.
    HttpServer::new(move || 
        // Create an instance of the app.
        App::new() 
        .wrap(Logger::default())
        .app_data(pool.clone())
            .route("/", web::get().to(index))
        // Route the index function to the root path.
            .service( web::scope("/products")
            .service( handlers::products::index )
            .service( handlers::products::get )
            .service( handlers::products::update )
            .service( handlers::products::create )
            .service( handlers::products::destroy ) )   
    )
    // bind to port 8080 and run the server on "127.0.0.1"
        .bind(("127.0.0.1", 8088))?
        .run()
        .await
}
