mod config;
mod controllers;
mod database;
mod helpers;
mod middleware;
mod models;
mod object;
mod routes;
mod validators;

use actix_multipart::form::tempfile::TempFileConfig;
use actix_web::{dev::Service, middleware::Logger, App, HttpServer};
use config::cors::create_cors;
use database::files::upload_files;
use futures_util::future::FutureExt;

use routes::{
    auth_routes::auth_routes, data_routes::data_routes, text_annotation_routes::annotation_routes,
    user_routes::user_routes,
};

use crate::middleware::auth_middleware::use_auth_middleware;

#[macro_use]
extern crate lazy_static;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(move || {
        App::new()
            .service(user_routes())
            .service(auth_routes())
            .service(annotation_routes())
            .service(data_routes())
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(upload_files)
            .wrap_fn(|req, srv| {
                use_auth_middleware(&req);

                srv.call(req).map(|res| res)
            })
            .wrap(Logger::default())
            .wrap(create_cors())
    })
    .bind(("127.0.0.1", 8000))?
    .workers(2)
    .run()
    .await
}
