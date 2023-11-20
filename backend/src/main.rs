mod config;
mod controllers;
mod database;
mod helpers;
mod models;
mod object;
mod routes;
mod validators;

use actix_web::{middleware::Logger, App, HttpServer};
use config::cors::create_cors;
use routes::{auth_routes::auth_routes, user_routes::user_routes};

#[macro_use]
extern crate lazy_static;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(create_cors())
            .wrap(Logger::default())
            .service(user_routes())
            .service(auth_routes())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
