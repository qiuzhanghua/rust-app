#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

// use log::*;
use actix_web::{middleware, web, App, HttpRequest, HttpServer, HttpResponse};
use actix_web_static_files;

use std::collections::HashMap;
use handlebars::Handlebars;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn index(_req: HttpRequest) -> &'static str {
//    debug!("REQ: {:?}", req);
    "Hello world!"
}

async fn index2() -> HttpResponse {
    HttpResponse::Ok().body("Hello world again!")
}


#[get("/{user}/{data}")]
async fn user(
    hb: web::Data<Handlebars>,
    info: web::Path<(String, String)>,
) -> HttpResponse {
    let data = json!({
        "user": info.0,
        "data": info.1
    });
    let body = hb.render("user", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
//    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();
//    std::env::set_var("RUST_LOG", "actix_web=info");
//    env_logger::init();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .service(actix_web_static_files::ResourceFiles::new(
                "/public", generate(),
            ))
            .app_data(handlebars_ref.clone())
            .service(web::resource("/").to(index))
            .service(web::resource("/again").to(index2))
            .service(user)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::{http, test, web, App, Error};

    #[actix_rt::test]
    async fn test_index() -> Result<(), Error> {
        let app = App::new().route("/", web::get().to(index));
        let mut app = test::init_service(app).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(response_body, r##"Hello world!"##);

        Ok(())
    }
}