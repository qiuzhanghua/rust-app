use actix_web::{web, HttpRequest, HttpResponse};
use crate::auth::{AuthService, SignupVo, LoginVo, AccountService};
use actix_web::http::header::AUTHORIZATION;
use actix_web::http::StatusCode;
use std::sync::{Mutex, Arc};

// POST api/auth/signup
#[post("/api/auth/signup")]
pub async fn signup(signup_vo: web::Json<SignupVo>, service: web::Data<Arc<Mutex<AccountService>>>) -> HttpResponse {
    let mut service = service.lock().unwrap();
    match service.signup(signup_vo.0) {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(err) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err),
    }
}

// POST api/auth/login
#[post("/api/auth/login")]
pub fn login(login_vo: web::Json<LoginVo>, service: web::Data<Arc<Mutex<AccountService>>>) -> HttpResponse {
    let mut service = service.lock().unwrap();
    match service.login(login_vo.0) {
        Ok(token_res) => HttpResponse::Ok().body(token_res),
        Err(err) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(err)
    }
}

// POST api/auth/logout
#[post("/api/auth/logout")]
pub fn logout(req: HttpRequest, service: web::Data<Arc<Mutex<AccountService>>>) -> HttpResponse {
    let mut service = service.lock().unwrap();
    if let Some(authen_header) = req.headers().get(AUTHORIZATION) {
        service.logout(authen_header);
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::BadRequest().finish()
    }
}
