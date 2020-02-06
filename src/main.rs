#[macro_use]
extern crate lazy_static;
//#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate viperus;
#[macro_use]
extern crate diesel;

// use log::*;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_static_files;
use std::collections::HashMap;
use handlebars::Handlebars;

mod utils;
mod config;
mod db;
mod handlers;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();
//    std::env::set_var("RUST_LOG", "actix_web=info");
//    env_logger::init();
    config::ok();
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
//            .data(web::JsonConfig::default().limit(4096))
            .service(actix_web_static_files::ResourceFiles::new(
                "/public", generate(),
            ))
            .app_data(handlebars_ref.clone())
            .service(web::resource("/")
                .route(web::get().to(handlers::index))
                .route(web::post().to(handlers::index_post)))
            .service(web::resource("/file.html")
                .route(web::get().to(handlers::select_file))
                .route(web::post().to(handlers::save_file)))
            .service(web::resource("/again").to(handlers::index2))
            .service(handlers::disp_user)
            .service(handlers::delete_user)
            .service(handlers::put_user)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

