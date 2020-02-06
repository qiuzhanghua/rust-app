use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest, HttpResponse, web, Error};
use handlebars::Handlebars;
use actix_multipart::Multipart;
use futures::StreamExt;
use async_std::prelude::*;
use crate::utils;
use crate::db::*;
use crate::db::models::User;

pub async fn index(_req: HttpRequest) -> &'static str {
//    debug!("REQ: {:?}", req);
    "Hello world!"
}


pub async fn index2() -> HttpResponse {
    HttpResponse::Ok().body("Hello world again!")
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserVo {
    pub name: String,
    pub email: String,
    pub enabled: bool,
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/PUT
#[put("/users")]
pub fn put_user(item: web::Json<UserVo>) -> HttpResponse {
    let u = create_user(&item.name, &item.email, item.enabled);
    if u.is_some() {
        let body = serde_json::to_string(&u.unwrap()).unwrap();
        HttpResponse::Ok().body(body)
    } else {
        HttpResponse::NotAcceptable().finish()
    }
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Methods/DELETE
#[delete("/users/{id}")]
pub async fn delete_user(info: web::Path<(i32, )>) -> HttpResponse {
    let id = info.0;
    if let Ok(_) = del_user(id) {
        HttpResponse::NoContent().finish() }
    else {
        HttpResponse::BadRequest().finish()
    }
}

#[get("/users/{id}.{ext}")]
pub async fn disp_user(hb: web::Data<Handlebars>,
                       // req: HttpRequest,
                       info: web::Path<(String, String)>,
) -> HttpResponse {
    let id = info.0.parse::<i32>();
    let u: Result<User, diesel::result::Error> = get_user(id.unwrap());
    let mut body = "{}".to_string();
    if info.1 == "html" {
        if u.is_ok() {
            let us = u.unwrap();
            body = hb.render("user", &us).unwrap();
            HttpResponse::Ok().body(body)
        } else {
            HttpResponse::Ok().content_type("application/json").body(body)
        }
    } else {
        if u.is_ok() {
            let us = u.unwrap();
            let body = serde_json::to_string(&us).unwrap();
            HttpResponse::Ok().body(body)
        } else {
            HttpResponse::Ok().content_type("application/json").body(body)
        }
    }
}


pub fn select_file() -> HttpResponse {
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

pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = content_type
            .get_filename()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete)?;
        let filename = utils::dnc_unicode(filename);
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
pub struct MyObj {
    pub name: String,
    pub number: i32,
}

// const MAX_SIZE: usize = 262_144; // max payload size is 256k

pub async fn index_post(item: web::Json<MyObj>) -> HttpResponse {
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