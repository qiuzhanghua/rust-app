#[macro_use]
extern crate lazy_static;

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
use async_std::prelude::*;

use std::collections::HashMap;
use handlebars::Handlebars;
use actix_web::web::route;
use actix_multipart::Multipart;
use regex::Captures;
use regex::Regex;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn index(_req: HttpRequest) -> &'static str {
//    debug!("REQ: {:?}", req);
    "Hello world!"
}

fn select_file() -> HttpResponse {
    let html = r###"<html>
        <head><title>Select file to upload</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"###;

    HttpResponse::Ok().body(html)
}

async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
//        println!("content type : {:?}", content_type);
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = dnc_unicode(filename);
        let filepath = format!("./log/{}", filename);
        let mut f = async_std::fs::File::create(filepath).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.write_all(&data).await?;
        }
    }
    Ok(HttpResponse::Ok().body("OK, uploaded!".to_string()))
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
                .route(web::post().to(index_post)))
            .service(web::resource("/file.html")
                .route(web::get().to(select_file))
                .route(web::post().to(save_file)))
            .service(web::resource("/again").to(index2))
            .service(user)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


// decimal numeric character to unicode
pub fn dnc_unicode(str: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r##"(&#(\d{1,5});)"##).unwrap();
    }
    RE.replace_all(str, |caps: &Captures| { format!("{}", std::char::from_u32(caps[2].parse::<u32>().ok().unwrap()).unwrap()) }).to_string()
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