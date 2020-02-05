#[macro_use]
extern crate actix_web;

#[macro_use]
extern crate serde_json;

// use log::*;
use json::JsonValue;
use serde::{Deserialize, Serialize};
use actix_web::{error, middleware, web, App, HttpRequest, HttpServer, HttpResponse, HttpMessage, Error};
use actix_web_static_files;
use bytes::{Bytes, BytesMut};
use futures::StreamExt;

use std::collections::HashMap;
use handlebars::Handlebars;
use actix_web::web::route;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn index(_req: HttpRequest) -> &'static str {
//    debug!("REQ: {:?}", req);
    "Hello world!"
}

#[derive(Debug, Serialize, Deserialize)]
struct MyObj {
    name: String,
    number: i32,
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

async fn index_post(item: web::Json<MyObj>) -> HttpResponse {
//, item: web::Json<MyObj>
    // mut payload: web::Payload
//    println!("{:?}", req);
//    println!("{:?}", item);
    HttpResponse::Ok().json(item.0)
//    HttpResponse::Ok().body("What")
    // body is loaded, now we can deserialize json-rust
//    let result = json::parse(std::str::from_utf8(&body).unwrap()); // return Result
//    let injson: JsonValue = match result {
//        Ok(v) => v,
//        Err(e) => json::object! {"err" => e.to_string() },
//    };
//    Ok(HttpResponse::Ok()
//        .content_type("application/json")
//        .body(injson.dump()))
    // payload is a stream of Bytes objects
//    let mut body = BytesMut::new();
//    while let Some(chunk) = payload.next().await {
//        let chunk = chunk?;
//        // limit max size of in-memory payload
//        if (body.len() + chunk.len()) > MAX_SIZE {
//            return Err(error::ErrorBadRequest("overflow"));
//        }
//        body.extend_from_slice(&chunk);
//    }
//
//    // body is loaded, now we can deserialize serde-json
//    let obj = serde_json::from_slice::<MyObj>(&body)?;
//    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

async fn index2() -> HttpResponse {
    HttpResponse::Ok().body("Hello world again!")
}


#[get("/{user}/{data}.{ext}")]
async fn user(hb: web::Data<Handlebars>,
              // req: HttpRequest,
              info: web::Path<(String, String, String)>,
) -> HttpResponse {
//    println!("{:?}", req.headers().get("accept".to_string()).unwrap());
    let data = json!({
        "user": info.0,
        "data": info.1
    });
    if info.2 == "html" {
        let body = hb.render("user", &data).unwrap();
        HttpResponse::Ok().body(body)
    } else {
        HttpResponse::Ok().json(data)
    }
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
//            .data(web::JsonConfig::default().limit(4096))
            .service(actix_web_static_files::ResourceFiles::new(
                "/public", generate(),
            ))
            .app_data(handlebars_ref.clone())
//            .service(web::resource("/").to(index)            )
            .service(web::resource("/")
                .route(web::get().to(index))
                .route(web::post().to(index_post)
                ))
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