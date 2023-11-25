mod config;
mod controllers;
mod database;
mod helpers;
mod middleware;
mod models;
mod object;
mod routes;
mod validators;

use actix_web::{dev::Service, middleware::Logger, App, HttpServer};
use config::cors::create_cors;
use futures_util::future::FutureExt;

use routes::{
    auth_routes::auth_routes, text_annotation_routes::annotation_routes, user_routes::user_routes,
};

use crate::middleware::auth_middleware::use_auth_middleware;

#[macro_use]
extern crate lazy_static;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(user_routes())
            .service(auth_routes())
            .service(annotation_routes())
            .wrap_fn(|req, srv| {
                use_auth_middleware(&req);

                srv.call(req).map(|res| res)
            })
            .wrap(Logger::default())
            .wrap(create_cors())
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
